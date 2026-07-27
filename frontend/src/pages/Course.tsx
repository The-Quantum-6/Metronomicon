import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import Navbar from "../components/Navbar";
import ContributeAlert from "../components/ContributeAlert";
import Contribute, {type ContributionType, type ContributeMode} from "../components/Contribute";
import ReportForm from "../components/forms/RaportForm";
import { apiUrl } from "../config";
import { useCurrentUser } from "../hooks/useCurrentUser";

const DISCLAIMER_SEEN = "metronomicon_policy_acknowledged"

type Course = {
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

type Resource = {
  resource_id: string
  title: string
  key: string
  status: string
  officiality: string
}

type Link = {
  link_id: string
  status: string
  label: string
  url: string
  official: boolean
}

type ProjectIdea = {
  idea_id: string
  title: string
  body: string
  difficulty: "Easy" | "Medium" | "Hard"
  status: string
}

type Faq = {
  faq_id: string
  question: string
  answer: string
  officiality: string
  status: string
}

type CourseTab = "overview" | "resources" | "links" | "projects" | "faqs";

const tabs = [
  { id: "overview"  as CourseTab, label: "Overview" },
  { id: "resources" as CourseTab, label: "Resources" },
  { id: "links"     as CourseTab, label: "Links" },
  { id: "projects"  as CourseTab, label: "Projects" },
  { id: "faqs"       as CourseTab, label: "FAQs" },
];

export default function Course() {
  const navigate = useNavigate();
  const { id } = useParams();
  const [course, setCourse] = useState<Course | null>(null);
  const [tab, setTab] = useState<CourseTab>("overview");
  const [showAlert, setShowAlert] = useState(false);
  const [showContribute, setShowContribute] = useState(false);
  const [contributeMode, setContributeMode] = useState<ContributeMode>("propose");
  const [preselecteType, setPreselectedType] = useState<ContributionType | null>(null);
  const { isStaff } = useCurrentUser();
  const [openFaqId, setOpenFaqId] = useState<string | null>(null);
   const [showReport, setShowReport] = useState(false);

  useEffect(() => {
  fetch(apiUrl(`courses/${id}`))
    .then((r) => r.json())
    .then((data) => setCourse(data))
    .catch(console.error);
}, [id]);

  const handleContribute = (mode: ContributeMode = "propose") => {
    setContributeMode(mode);
    if (localStorage.getItem(DISCLAIMER_SEEN)){
      setShowContribute(true);
    }
    else {
      setShowAlert(true)
    }
  }

  if (!course) return null;

return (
  <>
    <Navbar />
    <main className="max-w-7xl mx-auto px-4 sm:px-6 py-8">
    <div className="text-[#6B6B5A] hover:text-[#1A1F3A] transition-colors">
      <button
      onClick={() => navigate("/")}
      className="inline-flex items-center gap-1.5 py-4 font-sans"><svg className="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
        <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M13 5H1m0 0 4 4M1 5l4-4"/>
        </svg>Back to courses</button>
        </div>
      <div className="border-b border-[#F4F2EB]">
        <p className="font-mono text-m text-[#6B6B5A]">
          {course.code}
        </p>
        <div className="flex items-start justify-between gap-4 flex-wrap pb-4 border-b border-[#DAD8D6]">
          <h1 className="text-4xl font-semibold font-display text-[#1A1F3A]">
            {course.name}
          </h1>
          <div className="flex gap-2">
            <button onClick={() => handleContribute("propose")} className="flex items-center px-4 py-2 rounded-lg bg-[#1A1F3A] text-white font-medium hover:opacity-90 transition-colors">
              + Contribute
            </button>
            {isStaff && (
              <button onClick={() => handleContribute("direct")} className="flex items-center px-4 py-2 rounded-lg bg-transparent text-[#1A1F3A] font-medium border border-[#1A1F3A] hover:bg-gray-100 transition-colors">
                + Staff Contribute
              </button>
            )}
            <button onClick={() => setShowReport(true)}
            className="flex items-center gap-1.5 px-4 py-2 text-[#6B6B5A] rounded-lg bg-transparent border border-[#6B6B5A] hover:bg-gray-100 transition-colors">
              Report
            </button>
          </div>
        </div>
      </div>

      <div className="flex overflow-x-auto pt-3 pb-3">
        {tabs.map((t) => (
          <button
            key={t.id}
            onClick={() => setTab(t.id)}
            className={`px-4 py-2 font-sans transition-colors rounded-lg
              ${tab === t.id ? "text-[#1A1F3A] bg-[#E3E1DD] border-[#F4F2EB]"
                : "text-[#6B6B5A] rounded-lg hover:bg-gray-200"}`}>
            {t.label}
          </button>
        ))}
      </div>

      {tab === "overview"  && 
      <div className="bg-white border border-[#DAD8D6] rounded-2xl p-5 max-w-4xl w-full">
        <h2 className="font-bold font-display text-xl pb-2">About this course</h2>
        <p className="text-[#6B6B5A]">{course.description}</p>
      </div>
      }
      
      
      {tab === "resources" && (
        <div>
          {course.resources.map((resource) => (
            <a key={resource.resource_id}
            href={resource.key}
            target="_blank"
            className="block border border-[#DAD8D6] rounded-xl p-3 mb-3 hover:bg-gray-200 transition-colors">
              <div className="flex justify-between items-start">
                <div>
                  <h3 className="font-semibold text-lg text-[#1A1F3A]">{resource.title}</h3>
                  <p className="text-base italic text-[#6B6B5A]">{resource.key}</p>
                  </div>

                  {resource.officiality == "Official" && (
                    <p className="text-sm px-2 py-1 border border-[#1A1F3A] rounded-full text-[#1A1F3A]">Official</p>
                    )}
                    </div>
                    </a>
                  ))}
                  <button onClick={() => { setPreselectedType("resource"); handleContribute();
                  }}
                  className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-lg text-[#6B6B5A] w-full hover:bg-gray-200">
                    + Add resource
                    </button>
                    </div>
                  )}

      {tab === "links" && (
        <div>
          {course.links.map((link) => (
            <a key={link.link_id} href={link.url} target="_blank"
            className="block border border-[#DAD8D6] rounded-xl p-3 mb-3 hover:bg-gray-200 transition-colors">
              <div className="flex justify-between items-start">
                <div>
                  <h3 className="font-semibold text-lg text-[#1A1F3A]">{link.label}</h3>
                  <p className="text-base italic text-[#6B6B5A] hover:underline">{link.url}</p>
                  </div>
                  {link.official && (
                    <p className="text-sm px-2 py-1 border border-[#1A1F3A] rounded-full text-[#1A1F3A]">Official</p>
                    )}
                    </div>
                    </a>
                  ))}
                  <button
                  onClick={() => {
                    setPreselectedType("link");
                    handleContribute();
                  }}
                  className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-lg text-[#6B6B5A] w-full hover:bg-gray-200">
                    + Add link
                    </button>
                    </div>
                  )}
      
      {tab === "projects" && (
        <div>
          {course.project_ideas.map((idea) => (
            <div key={idea.idea_id}
            className="border border-[#DAD8D6] rounded-xl p-4 mb-3">
              <div className="flex justify-between items-start">
                <div>
                  <h3 className="font-semibold text-lg text-[#1A1F3A]">{idea.title}</h3>
                  <p className="text-base text-[#6B6B5A] mt-2">{idea.body}</p>
                  </div>
                  <p className="text-sm px-3 py-1 rounded-full border border-[#DAD8D6] text-[#6B6B5A]">{idea.difficulty}</p>
                  </div>
                  </div>
                ))}
                
                <button onClick={() => {
                  setPreselectedType("project_idea");
                  handleContribute();
                }}
                className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-lg text-[#6B6B5A] w-full hover:bg-gray-200">+ Add project idea</button>
                </div>
              )}
      {tab === "faqs" && (
        <div>
          {course.faqs.map((faq) => {
            const isOpen = openFaqId === faq.faq_id;
            
            return (
            <div key={faq.faq_id} className="border border-[#DAD8D6] rounded-xl mb-2 overflow-hidden">
              <button onClick={() => setOpenFaqId(isOpen ? null : faq.faq_id)}
              className={`relative flex justify-between items-center w-full text-left px-4 py-3 hover:bg-[#F4F2EB] transition-colors ${isOpen ? "after:absolute after:bottom-0 after:left-4 after:right-4 after:border-b after:border-[#DAD8D6]" : ""}`}>
                <h3 className="font-medium text-lg text-[#1A1F3A]">{faq.question}</h3>
                <div className="flex items-center gap-2">{faq.officiality === "Official" && (
                  <p className="text-sm px-2 py-1 border border-[#1A1F3A] rounded-full text-[#1A1F3A]">Official</p>)}
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"
                  className={`w-5 h-5 text-[#6B6B5A] transition-transform duration-200 ${isOpen ? "rotate-180" : ""}`}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M19 9l-7 7-7-7"/>
                    </svg>
                    </div>
                    </button>
            {isOpen && (
              <div className="px-4 pb-4 pt-1 border-t border-[#F4F2EB]">
                <p className="text-base text-[#6B6B5A]">{faq.answer}</p>
                </div>
              )}
              </div>
              );
              })}
              <button onClick={() => { setPreselectedType("faq"); handleContribute(); }}
              className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 mt-2 text-lg text-[#6B6B5A] w-full hover:bg-gray-200">+ Add FAQ
              </button>
              </div>
            )}
    </main>

    {showAlert && (
      <ContributeAlert onConfirm={() => {setShowAlert(false); setShowContribute(true)}} onCancel={() => setShowAlert(false)}
      />
    )}

    {showContribute && (
          <Contribute
          courseId={id ?? ""}
          mode={contributeMode}
          preselected={preselecteType}
      onCancel={() => {setShowContribute(false); setPreselectedType(null)}}
        />
    )}
    
    {showReport && (
      <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/40">
        <div className="bg-white rounded-2xl p-7 max-w-lg w-full mx-4 shadow-md text-[#1A1F3A]">
          <h2 className="text-2xl font-semibold font-display mb-6">Report an issue</h2>
          <ReportForm courseId={id} onCancel={() => setShowReport(false)} />
          </div>
          </div>
        )}
  </>
);
}