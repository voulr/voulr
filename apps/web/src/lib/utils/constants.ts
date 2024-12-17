import { env } from "$env/dynamic/public"

export const apiUrl = new URL(env.PUBLIC_API_URL)
export const githubLoginUrl = new URL(`${apiUrl.origin}/github/login`)
