export type Course = {
  name: string
  code: string
  field: string
  description?: string | null
  tags: string[]
  resources: Resource[]
  links: Link[]
  project_ideas: ProjectIdea[]
  faqs: Faq[]
}

export type Resource = {
  resource_id: string
  title: string
  key: string
  status: string
  officiality: string
}

export type Link = {
  link_id: string
  status: string
  label: string
  url: string
  official: boolean
}

export type ProjectIdea = {
  idea_id: string
  title: string
  body: string
  difficulty: "Easy" | "Medium" | "Hard"
  status: string
}

export type Faq = {
  faq_id: string
  question: string
  answer: string
  officiality: string
  status: string
}

export type CourseTab = "overview" | "resources" | "links" | "project_ideas" | "faqs";