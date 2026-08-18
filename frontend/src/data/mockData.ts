// The contribution queue now reads live data — see src/api/contributions.ts.
// Only the report mocks are left here, until the report endpoints are wired up.

export type Uuid = string;

export type ReportCategory =
  | "BugOrTechnicalIssue"
  | "CopyrightOrSensitiveContent"
  | "PolicyConcern"
  | "Other";

export const reportCategoryLabels: Record<ReportCategory, string> = {
  BugOrTechnicalIssue: "Bug or technical issue",
  CopyrightOrSensitiveContent: "Copyright / sensitive content",
  PolicyConcern: "Policy concern",
  Other: "Other",
};

export interface ReportedContent {
  id: Uuid;
  category: ReportCategory;
  description: string;
  contactEmail?: string;
  createdAt: string; // ISO date
}

export const mockReportedContent: ReportedContent[] = [
  {
    id: "b1b2c3d4-0001-4a1a-9c1a-000000000011",
    category: "BugOrTechnicalIssue",
    description: "The 'Approve' button on the profile page doesn't respond on mobile Safari.",
    contactEmail: "kari.n@example.com",
    createdAt: "2026-07-11T10:02:00Z",
  },
  {
    id: "b1b2c3d4-0002-4a1a-9c1a-000000000012",
    category: "CopyrightOrSensitiveContent",
    description: "A project idea on the React Fundamentals course reposts a paid course's material verbatim.",
    createdAt: "2026-07-12T13:40:00Z",
  },
  {
    id: "b1b2c3d4-0003-4a1a-9c1a-000000000013",
    category: "PolicyConcern",
    description: "Users are posting affiliate links disguised as course resources.",
    contactEmail: "trond.i@example.com",
    createdAt: "2026-07-12T18:15:00Z",
  },
  {
    id: "b1b2c3d4-0004-4a1a-9c1a-000000000014",
    category: "Other",
    description: "Site footer overlaps content on very narrow viewports (<320px).",
    createdAt: "2026-07-13T07:55:00Z",
  },
];