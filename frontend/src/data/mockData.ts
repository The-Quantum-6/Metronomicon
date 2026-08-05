export type Uuid = string;

export type TextSuggestionKind =
  | { type: "AddLink"; course_id: Uuid; label: string; url: string }
  | { type: "EditLink"; link_id: Uuid; label?: string; url?: string }
  | { type: "RemoveLink"; link_id: Uuid }
  | { type: "AddFaqEntry"; course_id: Uuid; question: string; answer: string }
  | { type: "EditFaqEntry"; faq_id: Uuid; question?: string; answer?: string }
  | { type: "RemoveFaqEntry"; faq_id: Uuid }
  | { type: "AddProjectIdea"; course_id: Uuid; title: string; body: string }
  | { type: "EditProjectIdea"; idea_id: Uuid; title?: string; body?: string }
  | { type: "RemoveProjectIdea"; idea_id: Uuid };

export interface Suggestion {
  id: Uuid;
  kind: TextSuggestionKind;
  authorName: string;
  courseName: string;
  createdAt: string; 
}

export function formatSuggestion(kind: TextSuggestionKind): {
  title: string;
  detail: string;
} {
  switch (kind.type) {
    case "AddLink":
      return { title: "Add link", detail: `${kind.label} → ${kind.url}` };
    case "EditLink":
      return {
        title: "Edit link",
        detail: [kind.label && `label: ${kind.label}`, kind.url && `url: ${kind.url}`]
          .filter(Boolean)
          .join(", "),
      };
    case "RemoveLink":
      return { title: "Remove link", detail: `link_id: ${kind.link_id}` };
    case "AddFaqEntry":
      return { title: "Add FAQ entry", detail: kind.question };
    case "EditFaqEntry":
      return {
        title: "Edit FAQ entry",
        detail: kind.question ?? kind.answer ?? "",
      };
    case "RemoveFaqEntry":
      return { title: "Remove FAQ entry", detail: `faq_id: ${kind.faq_id}` };
    case "AddProjectIdea":
      return { title: "Add project idea", detail: kind.title };
    case "EditProjectIdea":
      return { title: "Edit project idea", detail: kind.title ?? kind.body ?? "" };
    case "RemoveProjectIdea":
      return { title: "Remove project idea", detail: `idea_id: ${kind.idea_id}` };
  }
}

export const mockSuggestions: Suggestion[] = [
  {
    id: "a1b2c3d4-0001-4a1a-9c1a-000000000001",
    authorName: "Kari Nordmann",
    courseName: "Introduction to Rust",
    createdAt: "2026-07-10T09:15:00Z",
    kind: {
      type: "AddLink",
      course_id: "c1000000-0000-0000-0000-000000000001",
      label: "Official Rust Book",
      url: "https://doc.rust-lang.org/book/",
    },
  },
  {
    id: "a1b2c3d4-0002-4a1a-9c1a-000000000002",
    authorName: "Ola Hansen",
    courseName: "Web Accessibility Basics",
    createdAt: "2026-07-11T14:32:00Z",
    kind: {
      type: "EditFaqEntry",
      faq_id: "f1000000-0000-0000-0000-000000000002",
      answer:
        "WCAG 2.2 adds new success criteria around focus visibility and drag alternatives.",
    },
  },
  {
    id: "a1b2c3d4-0003-4a1a-9c1a-000000000003",
    authorName: "Silje Berg",
    courseName: "React Fundamentals",
    createdAt: "2026-07-12T08:05:00Z",
    kind: {
      type: "AddProjectIdea",
      course_id: "c1000000-0000-0000-0000-000000000003",
      title: "Build a Kanban board",
      body: "A drag-and-drop task board using React DnD and local storage persistence.",
    },
  },
  {
    id: "a1b2c3d4-0004-4a1a-9c1a-000000000004",
    authorName: "Mohammed Ali",
    courseName: "Introduction to Rust",
    createdAt: "2026-07-12T16:47:00Z",
    kind: {
      type: "RemoveLink",
      link_id: "l1000000-0000-0000-0000-000000000004",
    },
  },
];