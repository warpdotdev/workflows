export interface Workflow {
  slug: WorkflowSlug;
  name: string;
  command: string;
  tags?: [string];
  description?: string;
  arguments?: [Argument];
  source_url?: string;
  author?: string;
  author_url?: string;
  shells?: [Shell];
}

export interface Argument {
  name: string;
  description?: string;
  default_value?: string;
}

export const enum Shell {
  Bash,
  Fish,
  Zsh,
}

export type WorkflowSlug = string;

function getFileNameOnly(filePath: string) : string {
  return filePath.split("/").pop()!.split(".")!.shift()!;
}

const requireContext = require.context(
  "yaml-loader!../specs",
  false,
  /\.ya?ml$/
);

export const WORKFLOWS = new Map<WorkflowSlug, Workflow>();

requireContext.keys().forEach((key: string) => {
  const obj = requireContext(key);
  const simpleKey = getFileNameOnly(key);
  let workflow = {
    ...obj,
    slug: simpleKey,
  } as Workflow;
  WORKFLOWS.set(simpleKey, workflow);
});