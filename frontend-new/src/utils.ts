// import marked from 'marked'
import axios, { AxiosResponse } from 'axios'

// const renderer = new marked.Renderer()

function sanitize (str: string) {
  return str.replace(/&<"/g, function (m) {
    if (m === '&') return '&amp;'
    if (m === '<') return '&lt;'
    return '&quot;'
  })
}

// renderer.image = function (src: string, title: string, alt) {
//   const exec = /=\s*(\d*)\s*x\s*(\d*)\s*$/.exec(title)
//   let res = '<img src="' + sanitize(src) + '" alt="' + sanitize(alt)
//   if (exec && exec[1]) res += '" width="' + exec[1]
//   if (exec && exec[2]) res += '" height="' + exec[2]
//   return res + '">'
// }

// marked.setOptions({
//   renderer: renderer,
// })

function getFromServer<T = any, R = AxiosResponse<T>> (path: string): Promise<R> {
  return axios.get(`${process.env.VUE_APP_SERVER_URL}${path}`)
}

function getFromServerWithNoCache<T = any, R = AxiosResponse<T>> (path: string): Promise<R> {
  return axios.get(`${process.env.VUE_APP_SERVER_URL}${path}`, {
    headers: {
      'Cache-Control': 'no-cache,no-store,must-revalidate,max-age=-1,private',
    },
  })
}

function postToServer<T = any, R = AxiosResponse<T>> (path: string, data: any): Promise<R> {
  return axios.post(`${process.env.VUE_APP_SERVER_URL}${path}`, data)
}

export {
  getFromServer,
  getFromServerWithNoCache,
  postToServer,
}
