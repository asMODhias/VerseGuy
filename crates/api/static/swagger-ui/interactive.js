// Interactive local Swagger-like UI for offline use
// - renderInteractiveUI(url, dom_id) loads /openapi.yaml and renders a basic interactive UI
// - Provides a lightweight "Try it" capability for simple GET requests

(function (global) {
  'use strict';

  function el(tag, cls) {
    const e = document.createElement(tag);
    if (cls) e.className = cls;
    return e;
  }

  function textNode(text) { return document.createTextNode(text); }

  function fetchSpec(url) {
    return fetch(url).then(r => {
      if (!r.ok) throw new Error('failed to fetch ' + url);
      return r.text();
    });
  }

  // Very small YAML -> JSON heuristic for our OpenAPI: extract title and top-level paths
  function parseMinimalOpenAPI(text) {
    try {
      // Try JSON first
      if (text.trim().startsWith('{')) return JSON.parse(text);
    } catch (e) {
      // ignore
    }

    const lines = text.split('\n');
    const spec = { info: {}, paths: {} };
    let current = null;
    let inParameters = false;
    let inRequestBody = false;
    let inFormProperties = false;

    for (const raw of lines) {
      const line = raw;
      const t = line.trim();
      if (t.startsWith('title:')) {
        spec.info.title = t.split(':').slice(1).join(':').trim().replace(/^\"|\"$/g, '');
      }
      if (/^\/.+:$/.test(t)) {
        // path line like /health:
        const p = t.replace(':', '').trim();
        spec.paths[p] = { params: [], form: [] };
        current = p;
        inParameters = false;
        inRequestBody = false;
        inFormProperties = false;
        continue;
      }

      if (!current) continue;

      // detect parameters section under current path
      if (/^parameters:/.test(t)) {
        inParameters = true;
        inRequestBody = false;
        inFormProperties = false;
        continue;
      }

      if (inParameters) {
        const m = t.match(/^- name:\s*(\w[\w\-]*)/);
        if (m) {
          spec.paths[current].params.push(m[1]);
          continue;
        }
        // end parameters when we hit a non-indented block or next section
        if (/^[^\s-]/.test(t) || t.length === 0) {
          inParameters = false;
        }
      }

      // detect requestBody -> application/x-www-form-urlencoded -> properties
      if (/^requestBody:/.test(t)) {
        inRequestBody = true;
        inFormProperties = false;
        continue;
      }
      if (inRequestBody && /application\/x-www-form-urlencoded:/.test(t)) {
        inFormProperties = true;
        continue;
      }
      if (inFormProperties) {
        const m = t.match(/^\s*(\w[\w\-]*):/);
        if (m) {
          spec.paths[current].form.push(m[1]);
          continue;
        }
        // end properties block
        if (/^[^\s]/.test(line) && !/^\s+\w+:/.test(line)) {
          inFormProperties = false;
          inRequestBody = false;
        }
      }
    }
    return spec;
  }

  function render(spec, mount) {
    mount.innerHTML = '';
    const header = el('div', 'swagger-ui__header');
    const h = el('h1');
    h.appendChild(textNode((spec.info && spec.info.title) ? spec.info.title : 'API'));
    header.appendChild(h);

    // auth panel
    const authPanel = el('div', 'swagger-ui__auth');
    const authLabel = el('div', 'swagger-ui__label');
    authLabel.textContent = 'Auth:';
    authPanel.appendChild(authLabel);
    const authStatus = el('div', 'swagger-ui__auth_status');
    authStatus.id = 'swagger-ui__auth_status';
    authStatus.textContent = DEFAULT_AUTH_HEADER ? ('Bearer ' + DEFAULT_AUTH_HEADER) : 'No token set';
    authPanel.appendChild(authStatus);
    const btnCreds = el('button', 'swagger-ui__button');
    btnCreds.textContent = 'Fetch Client Credentials';
    btnCreds.onclick = async () => {
      const clientId = prompt('Client ID', 'demo');
      const clientSecret = prompt('Client Secret', 'secret');
      try {
        const tok = await fetchTokenClientCredentials(clientId, clientSecret);
        authStatus.textContent = 'Bearer ' + tok.access_token;
      } catch (e) {
        alert('token request failed: ' + e.message);
      }
    };
    authPanel.appendChild(btnCreds);
    const btnClear = el('button', 'swagger-ui__button');
    btnClear.textContent = 'Clear Token';
    btnClear.onclick = () => { setAuthHeader(undefined); authStatus.textContent = 'No token set'; };
    authPanel.appendChild(btnClear);

    header.appendChild(authPanel);
    mount.appendChild(header);

    const container = el('div', 'swagger-ui__interactive');

    const endpoints = el('div', 'swagger-ui__endpoints');
    Object.keys(spec.paths || {}).forEach(p => {
      const ep = el('div', 'swagger-ui__endpoint');
      const title = el('div', 'swagger-ui__endpoint__path');
      title.appendChild(textNode(p));
      ep.appendChild(title);

      // Details (parameters / form)
      const details = el('div', 'swagger-ui__details');
      const detailBtn = el('button', 'swagger-ui__button');
      detailBtn.textContent = 'Details';
      detailBtn.onclick = () => {
        const shown = details.style.display === 'block';
        details.style.display = shown ? 'none' : 'block';
      };
      ep.appendChild(detailBtn);

      // request method buttons
      const btnGet = el('button', 'swagger-ui__try');
      btnGet.textContent = 'Try GET';
      btnGet.onclick = () => showTryFormWithParams(p, 'GET', mount, spec.paths[p]);
      ep.appendChild(btnGet);

      const btnPost = el('button', 'swagger-ui__try swagger-ui__try--post');
      btnPost.textContent = 'Try POST';
      btnPost.onclick = () => showTryFormWithParams(p, 'POST', mount, spec.paths[p]);
      ep.appendChild(btnPost);

      const btnPut = el('button', 'swagger-ui__try swagger-ui__try--put');
      btnPut.textContent = 'Try PUT';
      btnPut.onclick = () => showTryFormWithParams(p, 'PUT', mount, spec.paths[p]);
      ep.appendChild(btnPut);

      // params container
      const paramsContainer = el('div', 'swagger-ui__params');
      paramsContainer.style.display = 'none';
      if (spec.paths[p].params && spec.paths[p].params.length) {
        const plist = el('div', 'swagger-ui__param_list');
        spec.paths[p].params.forEach(n => {
          const row = el('div', 'swagger-ui__param_row');
          const label = el('label'); label.textContent = n + ':';
          const input = el('input'); input.className = 'swagger-ui__param_input'; input.value = '';
          input.setAttribute('data-param-name', n);
          row.appendChild(label);
          row.appendChild(input);
          plist.appendChild(row);
        });
        paramsContainer.appendChild(plist);
      }
      if (spec.paths[p].form && spec.paths[p].form.length) {
        const fl = el('div', 'swagger-ui__form_list');
        spec.paths[p].form.forEach(n => {
          const row = el('div', 'swagger-ui__param_row');
          const label = el('label'); label.textContent = n + ':';
          const input = el('input'); input.className = 'swagger-ui__form_input'; input.value = '';
          input.setAttribute('data-form-name', n);
          row.appendChild(label);
          row.appendChild(input);
          fl.appendChild(row);
        });
        paramsContainer.appendChild(fl);
      }
      details.appendChild(paramsContainer);
      ep.appendChild(details);

      endpoints.appendChild(ep);
    });

    container.appendChild(endpoints);

    const responseArea = el('div', 'swagger-ui__response');
    responseArea.id = 'swagger-ui__response_area';
    container.appendChild(responseArea);

    mount.appendChild(container);
  }

  function showTryFormWithParams(path, method, mount, meta) {
    const area = mount.querySelector('#swagger-ui__response_area');
    area.innerHTML = '';

    const form = el('div', 'swagger-ui__try_form');
    const urlRow = el('div', 'swagger-ui__try_row');
    const input = el('input', 'swagger-ui__url_input');
    input.value = path;
    input.style.width = '60%';
    urlRow.appendChild(input);

    const methodLabel = el('span', 'swagger-ui__method_label');
    methodLabel.textContent = method;
    urlRow.appendChild(methodLabel);

    form.appendChild(urlRow);

    // Parameter fields
    const paramsDiv = el('div', 'swagger-ui__params_form');
    if (meta && meta.params && meta.params.length) {
      meta.params.forEach(n => {
        const row = el('div', 'swagger-ui__param_row');
        const label = el('label'); label.textContent = n + ':';
        const inputp = el('input'); inputp.className = 'swagger-ui__param_input'; inputp.value = '';
        inputp.setAttribute('data-param-name', n);
        row.appendChild(label); row.appendChild(inputp); paramsDiv.appendChild(row);
      });
    }
    if (meta && meta.form && meta.form.length) {
      meta.form.forEach(n => {
        const row = el('div', 'swagger-ui__param_row');
        const label = el('label'); label.textContent = n + ':';
        const inputp = el('input'); inputp.className = 'swagger-ui__form_input'; inputp.value = '';
        inputp.setAttribute('data-form-name', n);
        row.appendChild(label); row.appendChild(inputp); paramsDiv.appendChild(row);
      });
    }
    form.appendChild(paramsDiv);

    // Headers editor
    const headersLabel = el('div', 'swagger-ui__label'); headersLabel.textContent = 'Headers (JSON):';
    const headersInput = el('textarea', 'swagger-ui__headers_input');
    headersInput.value = JSON.stringify({ 'Content-Type': 'application/json' }, null, 2);
    headersInput.rows = 3;
    headersInput.style.width = '80%';
    form.appendChild(headersLabel);
    form.appendChild(headersInput);

    const go = el('button', 'swagger-ui__go');
    go.textContent = 'Send ' + method;

    async function sendRequest() {
      let url = input.value;
      // collect query params
      const qparts = [];
      paramsDiv.querySelectorAll('[data-param-name]').forEach(elm => {
        const name = elm.getAttribute('data-param-name');
        const v = elm.value;
        if (v && v.length) qparts.push(`${encodeURIComponent(name)}=${encodeURIComponent(v)}`);
      });
      if (qparts.length) {
        const sep = url.includes('?') ? '&' : '?';
        url = url + sep + qparts.join('&');
      }

      // headers
      let headers = {};
      try { headers = JSON.parse(headersInput.value || '{}'); } catch (e) { area.innerHTML = '<div class="swagger-ui__response__error">Invalid headers JSON: '+e.message+'</div>'; return; }
      if (DEFAULT_AUTH_HEADER) headers['Authorization'] = DEFAULT_AUTH_HEADER;

      // body (form fields -> urlencoded)
      let body = undefined;
      if (method !== 'GET') {
        const formPairs = [];
        paramsDiv.querySelectorAll('[data-form-name]').forEach(elm => {
          const name = elm.getAttribute('data-form-name');
          const v = elm.value;
          if (v && v.length) formPairs.push(`${encodeURIComponent(name)}=${encodeURIComponent(v)}`);
        });
        if (formPairs.length) {
          body = formPairs.join('&');
          headers['Content-Type'] = 'application/x-www-form-urlencoded';
        }
      }

      area.innerHTML = '<div class="swagger-ui__loading">Sending request...</div>';
      try {
        const r = await fetch(url, { method, headers, body });
        const text = await r.text();
        area.innerHTML = '';
        const status = el('div', 'swagger-ui__response__status');
        status.appendChild(textNode('HTTP ' + r.status));
        area.appendChild(status);

        const hdrs = el('div', 'swagger-ui__response__headers');
        hdrs.appendChild(textNode('Headers: ' + JSON.stringify([...r.headers])));
        area.appendChild(hdrs);

        const bodyPre = el('pre', 'swagger-ui__response__body');
        try {
          const json = JSON.parse(text);
          bodyPre.textContent = JSON.stringify(json, null, 2);
        } catch (e) {
          bodyPre.textContent = text;
        }
        area.appendChild(bodyPre);
      } catch (err) {
        area.innerHTML = '';
        const errdiv = el('div', 'swagger-ui__response__error');
        errdiv.textContent = 'Request failed: ' + err.message;
        area.appendChild(errdiv);
      }
    }

    go.onclick = sendRequest;
    form.appendChild(go);

    // expose helper for tests and other scripts
    form.sendRequest = sendRequest;

    area.appendChild(form);
  }

  // Default Authorization header (optional) used when present
    let DEFAULT_AUTH_HEADER = undefined;

    async function fetchTokenClientCredentials(clientId, clientSecret) {
      const body = `grant_type=client_credentials&client_id=${encodeURIComponent(clientId)}&client_secret=${encodeURIComponent(clientSecret)}`;
      const r = await fetch('/oauth/token', { method: 'POST', headers: { 'Content-Type': 'application/x-www-form-urlencoded' }, body });
      if (!r.ok) throw new Error('token request failed: ' + r.status);
      const j = await r.json();
      if (!j.access_token) throw new Error('no access_token in response');
      setAuthHeader(j.access_token);
      return j;
    }

    function setAuthHeader(token) {
      if (!token) {
        DEFAULT_AUTH_HEADER = undefined;
      } else {
        DEFAULT_AUTH_HEADER = `Bearer ${token}`;
      }
    }

    // Patch sendRequest to include DEFAULT_AUTH_HEADER if present
    const _oldSendRequest = undefined; // placeholder - each form sets its own sendRequest, we will augment it at the point of creation

    // Expose helpers
    global.fetchTokenClientCredentials = fetchTokenClientCredentials;
    global.setAuthHeader = setAuthHeader;

    function renderInteractiveUI(url, dom_id) {
      const mount = document.querySelector(dom_id);
      if (!mount) throw new Error('dom_id not found: ' + dom_id);
      mount.innerHTML = '<div class="swagger-ui__loading">Loading API documentation...</div>';

      fetchSpec(url)
        .then(text => parseMinimalOpenAPI(text))
        .then(spec => render(spec, mount))
        .catch(err => { mount.innerHTML = '<div class="swagger-ui__error">Failed to load API spec: ' + err.message + '</div>'; });
    }

    // augment showTryForm to merge DEFAULT_AUTH_HEADER into headers if set
    const _orig_showTryForm = showTryForm;
    showTryForm = function(path, method, mount) {
      _orig_showTryForm(path, method, mount);
      const area = mount.querySelector('#swagger-ui__response_area');
      const form = area.querySelector('.swagger-ui__try_form');
      if (!form) return;
      const originalSend = form.sendRequest;
      form.sendRequest = async function() {
        // merge default auth header
        const headersTextArea = form.querySelector('.swagger-ui__headers_input');
        let headers = {};
        try { headers = JSON.parse(headersTextArea.value || '{}'); } catch (e) { /* keep as-is, send will produce error */ }
        if (DEFAULT_AUTH_HEADER) {
          headers['Authorization'] = DEFAULT_AUTH_HEADER;
          headersTextArea.value = JSON.stringify(headers, null, 2);
        }
        return originalSend();
      };
    };

    // Authorization Code helper - opens popup, listens for postMessage from callback, exchanges code
    function performAuthorizationCodeFlow({ clientId, clientSecret, redirectUri = '/static/swagger-ui/oauth-callback.html', state = undefined, timeout = 60000 } = {}) {
      return new Promise((resolve, reject) => {
        if (!clientId || !clientSecret) return reject(new Error('clientId and clientSecret are required'));

        const s = state || Math.random().toString(36).substring(2);
        const authUrl = `/oauth/authorize?response_type=code&client_id=${encodeURIComponent(clientId)}&redirect_uri=${encodeURIComponent(redirectUri)}${s ? `&state=${encodeURIComponent(s)}` : ''}`;

        const popup = window.open(authUrl, 'oauthWindow', 'width=600,height=600');
        if (!popup) return reject(new Error('failed to open popup'));

        let finished = false;

        function onMessage(e) {
          try {
            const data = e.data || {};
            if (data && data.type === 'oauth_code' && data.code) {
              // exchange code
              const code = data.code;
              window.removeEventListener('message', onMessage);
              try { popup.close(); } catch (e) {}

              // build form body
              const body = `grant_type=authorization_code&code=${encodeURIComponent(code)}&redirect_uri=${encodeURIComponent(redirectUri)}&client_id=${encodeURIComponent(clientId)}&client_secret=${encodeURIComponent(clientSecret)}`;
              fetch('/oauth/token', { method: 'POST', headers: { 'Content-Type': 'application/x-www-form-urlencoded' }, body })
                .then(r => {
                  if (!r.ok) throw new Error('token exchange failed: ' + r.status);
                  return r.json();
                })
                .then(j => {
                  setAuthHeader(j.access_token);
                  finished = true;
                  resolve(j);
                })
                .catch(err => reject(err));
            }
          } catch (e) {
            // ignore
          }
        }

        window.addEventListener('message', onMessage);

        const timer = setTimeout(() => {
          if (!finished) {
            window.removeEventListener('message', onMessage);
            try { popup.close(); } catch(e) {}
            reject(new Error('authorization timed out'));
          }
        }, timeout);
      });
    }

    global.performAuthorizationCodeFlow = performAuthorizationCodeFlow;

    global.renderInteractiveUI = renderInteractiveUI;
  })(window);
