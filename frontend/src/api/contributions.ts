// Shapes and calls for the contribution moderation queue.
//
// The types mirror `backend/src/aggregates/contribution/command.rs`. Keep the
// variant names identical to the Rust enums — serde matches on them, so a typo
// here fails the request rather than the build.

import { apiUrl } from "../config";

export type Difficulty = "Easy" | "Medium" | "Hard";

/** Mirrors `ModerationVerdict`. The backend knows no other spelling. */
export type ModerationVerdict = "Approve" | "Deny";

export type TextContributionKind =
  | { AddLink: { label: string; url: string } }
  | { EditLink: { link_id: string; label: string | null; url: string | null } }
  | { RemoveLink: { link_id: string } }
  | { AddFaqEntry: { question: string; answer: string } }
  | { EditFaqEntry: { faq_id: string; question: string | null; answer: string | null } }
  | { RemoveFaqEntry: { faq_id: string } }
  | { AddProjectIdea: { title: string; body: string; difficulty: Difficulty } }
  | {
      EditProjectIdea: {
        idea_id: string;
        title: string | null;
        body: string | null;
        difficulty: Difficulty | null;
      };
    }
  | { RemoveProjectIdea: { idea_id: string } };

export type FileContributionKind =
  | { AddResource: { title: string; key: string } }
  | { RemoveResource: { resource_id: string } };

export type ContributionKind =
  | { Text: TextContributionKind }
  | { File: FileContributionKind };

export type ContributionStatus = "Proposed" | "Approved" | "Denied";

export type Contribution = {
  aggregate_id: string;
  course_id: string;
  contribution: ContributionKind;
  status: ContributionStatus;
  comment: string;
};

async function failure(response: Response): Promise<Error> {
  const body = await response.text().catch(() => "");
  return new Error(body || `Request failed with status ${response.status}`);
}

/**
 * Uploads a file and returns the storage key to reference it by. The key is
 * minted server-side, so the file is inert until a resource points at it.
 */
export async function uploadFile(file: File): Promise<string> {
  const form = new FormData();
  form.append("file", file);

  const response = await fetch(apiUrl("files"), {
    method: "POST",
    credentials: "include",
    body: form,
  });
  if (!response.ok) {
    // 413 is the body-limit layer rejecting the upload before the handler runs.
    if (response.status === 413) {
      throw new Error("The file is too large to upload.");
    }
    throw await failure(response);
  }

  const { key } = (await response.json()) as { key: string };
  return key;
}

/** Sends a contribution to the moderation queue. */
export async function proposeContribution(
  courseId: string,
  contribution: ContributionKind,
  comment: string,
): Promise<void> {
  const response = await fetch(apiUrl("contributions"), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify({
      Propose: { course_id: courseId, contribution, comment },
    }),
  });
  if (!response.ok) throw await failure(response);
}

export async function listContributions(): Promise<Contribution[]> {
  const response = await fetch(apiUrl("contributions"), { credentials: "include" });
  if (!response.ok) throw await failure(response);
  return (await response.json()) as Contribution[];
}

export async function moderateContribution(
  contributionId: string,
  verdict: ModerationVerdict,
): Promise<void> {
  const response = await fetch(apiUrl("contributions"), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify({
      Moderate: { contribution_id: contributionId, verdict },
    }),
  });
  if (!response.ok) throw await failure(response);
}

/** The storage key behind a file contribution, or null for text contributions. */
export function fileKeyOf(kind: ContributionKind): string | null {
  if ("File" in kind && "AddResource" in kind.File) {
    return kind.File.AddResource.key;
  }
  return null;
}

export function fileUrl(key: string): string {
  return apiUrl(`files/${encodeURIComponent(key)}`);
}

/** A short human-readable summary for the moderation queue. */
export function describeContribution(kind: ContributionKind): {
  title: string;
  detail: string;
} {
  if ("Text" in kind) {
    const text = kind.Text;
    if ("AddLink" in text)
      return { title: "Add link", detail: `${text.AddLink.label} → ${text.AddLink.url}` };
    if ("EditLink" in text)
      return { title: "Edit link", detail: text.EditLink.label ?? text.EditLink.url ?? "" };
    if ("RemoveLink" in text)
      return { title: "Remove link", detail: text.RemoveLink.link_id };
    if ("AddFaqEntry" in text)
      return { title: "Add FAQ entry", detail: text.AddFaqEntry.question };
    if ("EditFaqEntry" in text)
      return {
        title: "Edit FAQ entry",
        detail: text.EditFaqEntry.question ?? text.EditFaqEntry.answer ?? "",
      };
    if ("RemoveFaqEntry" in text)
      return { title: "Remove FAQ entry", detail: text.RemoveFaqEntry.faq_id };
    if ("AddProjectIdea" in text)
      return { title: "Add project idea", detail: text.AddProjectIdea.title };
    if ("EditProjectIdea" in text)
      return {
        title: "Edit project idea",
        detail: text.EditProjectIdea.title ?? text.EditProjectIdea.body ?? "",
      };
    return { title: "Remove project idea", detail: text.RemoveProjectIdea.idea_id };
  }

  const file = kind.File;
  if ("AddResource" in file)
    return { title: "Add resource", detail: file.AddResource.title };
  return { title: "Remove resource", detail: file.RemoveResource.resource_id };
}
