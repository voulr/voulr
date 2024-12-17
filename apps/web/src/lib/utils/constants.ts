import { PUBLIC_API_URL } from "$env/static/public"

export const apiUrl = new URL(PUBLIC_API_URL)
export const githubLoginUrl = new URL(`${apiUrl.origin}/github/login`)
