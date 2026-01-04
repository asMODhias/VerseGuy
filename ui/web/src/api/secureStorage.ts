type RequestMessage = { id: string; type: string; key: string; value?: string }

type ResponseMessage = { id: string; ok: boolean; value?: string; error?: string }

let pending: Map<string, (r: ResponseMessage) => void> = new Map()

function makeId() {
  return Math.random().toString(36).slice(2)
}

function postMessage(msg: RequestMessage) {
  if (window && (window as any).chrome && (window as any).chrome.webview && (window as any).chrome.webview.postMessage) {
    ;(window as any).chrome.webview.postMessage(msg)
    return true
  }
  return false
}

if (typeof window !== 'undefined' && (window as any).chrome && (window as any).chrome.webview && (window as any).chrome.webview.addEventListener) {
  ;(window as any).chrome.webview.addEventListener('message', (ev: any) => {
    const data: ResponseMessage = ev.data
    const cb = pending.get(data.id)
    if (cb) {
      cb(data)
      pending.delete(data.id)
    }
  })
}

export async function secureGet(key: string): Promise<string | null> {
  const id = makeId()
  const msg: RequestMessage = { id, type: 'secureStorage.get', key }

  if (!postMessage(msg)) {
    // fallback
    console.warn('[secureStorage] Native host not available; falling back to localStorage (not secure)')
    return Promise.resolve(localStorage.getItem(key))
  }

  return new Promise((resolve, reject) => {
    pending.set(id, (res) => {
      if (res.ok) resolve(res.value ?? null)
      else reject(new Error(res.error || 'unknown'))
    })
  })
}

export async function secureSet(key: string, value: string): Promise<void> {
  const id = makeId()
  const msg: RequestMessage = { id, type: 'secureStorage.set', key, value }

  if (!postMessage(msg)) {
    console.warn('[secureStorage] Native host not available; falling back to localStorage (not secure)')
    localStorage.setItem(key, value)
    return Promise.resolve()
  }

  return new Promise((resolve, reject) => {
    pending.set(id, (res) => {
      if (res.ok) resolve()
      else reject(new Error(res.error || 'unknown'))
    })
  })
}

export async function secureRemove(key: string): Promise<void> {
  const id = makeId()
  const msg: RequestMessage = { id, type: 'secureStorage.remove', key }

  if (!postMessage(msg)) {
    console.warn('[secureStorage] Native host not available; falling back to localStorage (not secure)')
    localStorage.removeItem(key)
    return Promise.resolve()
  }

  return new Promise((resolve, reject) => {
    pending.set(id, (res) => {
      if (res.ok) resolve()
      else reject(new Error(res.error || 'unknown'))
    })
  })
}
