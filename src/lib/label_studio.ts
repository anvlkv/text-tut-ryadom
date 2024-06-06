import type { LabelStudioSource } from "./types/AppData";

export async function labelStudioGet<T = {}>(path: string, config: LabelStudioSource) {
  const response =  await fetch(`${config.label_studio_url}/api/${path}`, { headers: { "Authorization": `Token ${config.token}` } })
  const data = await response.json()
  return data as T
}

export async function labelStudioPost<T = {}>(path: string, body: any, config: LabelStudioSource) {
  const response =  await fetch(`${config.label_studio_url}/api/${path}`, { headers: { "Authorization": `Token ${config.token}` }, method: 'POST', body })
  const data = await response.json()
  return data as T
}

export async function labelStudioPatch<T = {}>(path: string, body: any, config: LabelStudioSource) {
  const response =  await fetch(`${config.label_studio_url}/api/${path}`, { headers: { "Authorization": `Token ${config.token}` }, method: 'PATCH', body })
  const data = await response.json()
  return data as T
}