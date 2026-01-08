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
    for (const raw of lines) {
      const line = raw.trim();
      if (line.startsWith('title:')) {
        spec.info.title = line.split(':').slice(1).join(':').trim().replace(/^\"|\"$/g, '');
      }
      if (/^\/.+:$/.test(line)) {
        // path line like /health:
        const p = line.replace(':', '').trim();
        spec.paths[p] = { get: {} };
        current = p;
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
    mount.appendChild(header);

    const container = el('div', 'swagger-ui__interactive');

    const endpoints = el('div', 'swagger-ui__endpoints');
    Object.keys(spec.paths || {}).forEach(p => {
      const ep = el('div', 'swagger-ui__endpoint');
      const title = el('div', 'swagger-ui__endpoint__path');
      title.appendChild(textNode(p));
      ep.appendChild(title);

      const btnGet = el('button', 'swagger-ui__try');
      btnGet.textContent = 'Try GET';
      btnGet.onclick = () => showTryForm(p, 'GET', mount);
      ep.appendChild(btnGet);

      const btnPost = el('button', 'swagger-ui__try swagger-ui__try--post');
      btnPost.textContent = 'Try POST';
      btnPost.onclick = () => showTryForm(p, 'POST', mount);
      ep.appendChild(btnPost);

      const btnPut = el('button', 'swagger-ui__try swagger-ui__try--put');
      btnPut.textContent = 'Try PUT';
      btnPut.onclick = () => showTryForm(p, 'PUT', mount);
      ep.appendChild(btnPut);

      endpoints.appendChild(ep);
    });

    container.appendChild(endpoints);

    const responseArea = el('div', 'swagger-ui__response');
    responseArea.id = 'swagger-ui__response_area';
    container.appendChild(responseArea);

    mount.appendChild(container);
  }

  function showTryForm(path, method, mount) {
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

    // Headers editor
    const headersLabel = el('div', 'swagger-ui__label'); headersLabel.textContent = 'Headers (JSON):';
    const headersInput = el('textarea', 'swagger-ui__headers_input');
    headersInput.value = JSON.stringify({ 'Content-Type': 'application/json' }, null, 2);
    headersInput.rows = 3;
    headersInput.style.width = '80%';
    form.appendChild(headersLabel);
    form.appendChild(headersInput);

    // Body editor (for POST/PUT)
    const bodyLabel = el('div', 'swagger-ui__label'); bodyLabel.textContent = 'Body (JSON):';
    const bodyInput = el('textarea', 'swagger-ui__body_input');
    bodyInput.value = method === 'GET' ? '' : '{\n  "example": "value"\n}';
    bodyInput.rows = 6;
    bodyInput.style.width = '80%';
    form.appendChild(bodyLabel);
    form.appendChild(bodyInput);

    const go = el('button', 'swagger-ui__go');
    go.textContent = 'Send ' + method;

    async function sendRequest() {
      const url = input.value;
      let headers = {};
      try { headers = JSON.parse(headersInput.value || '{}'); } catch (e) { area.innerHTML = '<div class="swagger-ui__response__error">Invalid headers JSON: '+e.message+'</div>'; return; }
      let body = undefined;
      if (method !== 'GET') {
        try { body = bodyInput.value ? JSON.stringify(JSON.parse(bodyInput.value)) : undefined; } catch (e) { area.innerHTML = '<div class="swagger-ui__response__error">Invalid JSON body: '+e.message+'</div>'; return; }
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

  function renderInteractiveUI(url, dom_id) {
    const mount = document.querySelector(dom_id);
    if (!mount) throw new Error('dom_id not found: ' + dom_id);
    mount.innerHTML = '<div class="swagger-ui__loading">Loading API documentation...</div>';

    fetchSpec(url)
      .then(text => parseMinimalOpenAPI(text))
      .then(spec => render(spec, mount))
      .catch(err => { mount.innerHTML = '<div class="swagger-ui__error">Failed to load API spec: ' + err.message + '</div>'; });
  }

  global.renderInteractiveUI = renderInteractiveUI;
})(window);
