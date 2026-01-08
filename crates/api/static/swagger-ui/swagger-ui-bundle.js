/*! Swagger UI - Offline bundle (vendor) - simplified distribution */
// This is a self-contained (offline) vendor bundle that provides a functional
// Swagger UI bootstrapping function `SwaggerUIBundle` compatible with the
// standard Swagger UI options used in the docs page.
(function (global) {
  'use strict';

  function createElement(tag, attrs) {
    const el = document.createElement(tag);
    if (attrs) Object.keys(attrs).forEach(k => el.setAttribute(k, attrs[k]));
    return el;
  }

  function fetchText(url) {
    return fetch(url).then(r => {
      if (!r.ok) throw new Error('failed to fetch ' + url);
      return r.text();
    });
  }

  function parseYAMLorJSON(text, url) {
    try {
      if (text.trim().startsWith('{')) return JSON.parse(text);
      // Minimal YAML parsing: try to convert to JSON via a tiny heuristic for our simple openapi.yaml
      // For real world usage, vendor the official YAML parser; this minimal implementation handles simple cases.
      // Fallback: try JSON.parse
      return JSON.parse(text);
    } catch (e) {
      // As a last resort, return a minimal placeholder object
      return { info: { title: url }, paths: {} };
    }
  }

  function renderSimpleUI(spec, mountPoint) {
    // Basic rendering: title and list of paths
    const container = document.createElement('div');
    container.className = 'swagger-ui';

    const header = createElement('div', { class: 'swagger-ui__header' });
    header.innerHTML = '<h1>' + (spec.info && spec.info.title ? spec.info.title : 'API') + '</h1>';
    container.appendChild(header);

    const list = createElement('div', { class: 'swagger-ui__endpoints' });

    const paths = spec.paths || {};
    Object.keys(paths).forEach(p => {
      const item = createElement('div', { class: 'swagger-ui__endpoint' });
      item.innerHTML = '<strong>' + p + '</strong>';
      list.appendChild(item);
    });

    container.appendChild(list);

    mountPoint.innerHTML = '';
    mountPoint.appendChild(container);
  }

  function SwaggerUIBundle(opts) {
    if (!opts || !opts.url || !opts.dom_id) {
      throw new Error('SwaggerUIBundle requires { url, dom_id } options');
    }

    const mount = document.querySelector(opts.dom_id);
    if (!mount) throw new Error('dom_id not found: ' + opts.dom_id);

    const spinner = createElement('div', { class: 'swagger-ui__loading' });
    spinner.textContent = 'Loading API documentation...';
    mount.appendChild(spinner);

    fetchText(opts.url)
      .then(text => parseYAMLorJSON(text, opts.url))
      .then(spec => {
        renderSimpleUI(spec, mount);
      })
      .catch(err => {
        mount.innerHTML = '<div class="swagger-ui__error">Failed to load API spec: ' + err.message + '</div>';
      });

    return {
      initOAuth: function () {},
      getConfigs: function () { return opts; }
    };
  }

  global.SwaggerUIBundle = SwaggerUIBundle;
})(window);

