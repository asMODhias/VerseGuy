export type Organization = {
  id: string
  name: string
  tag: string
  member_count: number
}

export async function listOrgs(): Promise<Organization[]> {
  const res = await fetch('/v1/orgs');
  if (!res.ok) throw new Error(`failed to list orgs: ${res.status}`);
  const body = await res.json();
  return body.orgs as Organization[];
}

export async function createOrg(name: string, tag: string): Promise<Organization> {
  const res = await fetch('/v1/orgs', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ name, tag }),
  });
  if (!res.ok) throw new Error(`failed to create org: ${res.status}`);
  return res.json();
}