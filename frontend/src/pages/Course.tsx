import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { apiUrl } from "../config";

import Navbar from "../components/Navbar";
import type { Course, CourseTab, Resource, Link, ProjectIdea, Faq } from "../types/courseTypes";
import ContributeAlert from "../components/ContributeAlert";
import Contribute, {type ContributionType} from "../components/Contribute";
import GradeDistribution from "../components/GradeDistribution";
import ReportForm from "../components/forms/ReportForm";
import AdminView from "../components/AdminView";
import FaqForm from "../components/forms/FaqForm";
import { GetRoleFromCookie } from "../components/Perms";
import { useCoursePerms, PERMISSIONS } from "../components/Perms";
import LinkForm from "../components/forms/LinkForm";
import ProjectIdeaForm from "../components/forms/ProjectIdeaForm";
import ResourceForm from "../components/forms/ResourceForm";

const DISCLAIMER_SEEN = "metronomicon_policy_acknowledged"

export default function Course() {
  const navigate = useNavigate();
  const { id } = useParams();
  const [course, setCourse] = useState<Course | null>(null);
  const [tab, setTab] = useState<CourseTab>("overview");
  const [showAlert, setShowAlert] = useState(false);
  const [showContribute, setShowContribute] = useState(false);
  const [preselecteType, setPreselectedType] = useState<ContributionType | null>(null);
  const [openFaqId, setOpenFaqId] = useState<string | null>(null);
  const [showReport, setShowReport] = useState(false);

  const [mode, setMode] = useState<"create"|"edit"|"delete"|null>(null);
  const [activeForm, setActiveForm] = useState<"resource" | "link" | "project_idea" | "faq" | null>(null);
  const [selectedFaq, setSelectedFaq] = useState<Faq | null>(null);
  const role = GetRoleFromCookie(id ?? "");
  const [selectedResource, setSelectedResource] = useState<Resource | null>(null);
  const [selectedLink, setSelectedLink] = useState<Link | null>(null);
  const [selectedProjectIdea, setSelectedProjectIdea] = useState<ProjectIdea | null>(null);

  const [adminMode, setAdminMode] = useState(false);
  const { perms, hasPerm } = useCoursePerms(id);
  // TODO: replace with actual auth check
  const isAdmin = (hasPerm(PERMISSIONS.MODERATE_TEXT) || hasPerm(PERMISSIONS.MODERATE_FILE) || hasPerm(PERMISSIONS.PAGE_ADMIN) || hasPerm(PERMISSIONS.TRANSFER_PERMS) || role === "admin");


  useEffect(() => {
  fetch(apiUrl(`courses/${id}`))
    .then((r) => r.json())
    .then((data) => setCourse(data))
    .catch(console.error);

  fetch(apiUrl(`permissions/token?course_id=${id}`), {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
  }).catch(console.error);
  }, [id]);

  const handleContribute = () => {
    if (localStorage.getItem(DISCLAIMER_SEEN)){
      setShowContribute(true);
    }
    else {
      setShowAlert(true)
    }
  }

   const tabs = [
    { id: "overview" as CourseTab, label: "Overview" },
    { id: "resources" as CourseTab, label: "Resources" },
    { id: "links" as CourseTab, label: "Links" },
    ...(course?.project_ideas.length
      ? [{ id: "project_ideas" as CourseTab, label: "Project Ideas" }]
      : []),
    { id: "faqs" as CourseTab, label: "FAQs" },
  ];

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
        <div className="flex justify-between items-center">
          <p className="font-mono text-m text-[#6B6B5A]">
            {course.code}
            </p>
            {isAdmin && (
              <button onClick={() => setAdminMode(!adminMode)} 
              className={`px-4 py-2 mb-1 rounded-lg transition ${adminMode ? "bg-[#1A1F3A] text-white" : "border border-[#6B6B5A] text-[#6B6B5A]"}`}>
                {adminMode ? "Admin mode ON" : "Admin mode OFF"}
                </button>
              )}
              </div>
        <div className="flex items-start justify-between gap-4 flex-wrap pb-4 border-b border-[#DAD8D6]">
          <h1 className="text-4xl font-semibold font-display text-[#1A1F3A]">
            {course.name}
          </h1>
          <div className="flex gap-2">
            <button onClick={ handleContribute }className="flex items-center px-4 py-2 rounded-lg bg-[#1A1F3A] text-white font-medium hover:opacity-90 transition-colors">
              + Contribute
            </button>
            <button onClick={() => setShowReport(true)}
            className="flex items-center gap-1.5 px-4 py-2 text-[#6B6B5A] rounded-lg bg-transparent border border-[#6B6B5A] hover:bg-gray-100 transition-colors">
              Report
            </button>
          </div>
        </div>
        {adminMode && <AdminView />}
      </div>
      {!adminMode && (
        <>
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
      <div className="flex gap-5 items-start">
        <div className="bg-white border border-[#DAD8D6] rounded-2xl p-5 max-w-4xl w-full">
          <h2 className="font-bold font-display text-xl pb-2">About this course</h2>
          <p className="text-[#6B6B5A]">{course.description}</p>
        </div>
        <div className="w-full max-w-md">
          <GradeDistribution courseCode={course.code} />
        </div>
      </div>
      }
      
      
      {tab === "resources" && (
        <div>
          {course.resources.length === 0 ? (
            <p className="text-[#6B6B5A] text-lg py-4">No resources yet. Be the first to contribute! </p>
          ) : (
            <>
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
                    <div className="flex items-center gap-2">
                    {resource.official && (
                      <p className="text-sm px-2 py-1 border border-[#1A1F3A] rounded-full text-[#1A1F3A]">Official</p>
                    )}
                    <div className="flex gap-3">
                      <button onClick={() => {setActiveForm("resource"); setSelectedResource(resource); setMode("edit")}} 
                      className="text-[#6B6B5A] hover:text-green-600 transition-colors">
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M12 20h9" /><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4Z" />
                        </svg>
                        </button>
                        <button onClick={() => {setActiveForm("resource"); setSelectedResource(resource); setMode("delete") }}
                        className="text-[#6B6B5A] hover:text-red-600 transition-colors">
                          <svg xmlns="http://www.w3.org/2000/svg" width="18"height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="3 6 5 6 21 6" /><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                          </svg>
                          </button>
                          </div>
                          </div>
                  </div>
                </a>
              ))}
            </>
          )}
          <button
            onClick={() => { setActiveForm("resource"); setSelectedResource(null); setMode("create") }}
            className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-lg text-[#6B6B5A] w-full hover:bg-gray-200">
            + Add resource
          </button>
        </div>
      )}

      {tab === "links" && (
        <div>
          {course.links.length === 0 ? ( <p className="text-[#6B6B5A] text-lg py-4">No links yet. Be the first to contribute!</p>) 
          : (
            course.links.map((link) => (
            <div key={link.link_id}
            className="block border border-[#DAD8D6] rounded-xl p-3 mb-3 hover:bg-gray-200 transition-colors">
              <div className="flex justify-between items-start">
                <div>
                  <a href={link.url} target="_blank" className="flex-1">
                  <h3 className="font-semibold text-lg text-[#1A1F3A]">{link.label}</h3>
                  <p className="text-base italic text-[#6B6B5A] hover:underline">{link.url}</p>
                  </a>
                  </div>
                  <div className="flex items-center gap-2">
                  {link.official && (
                    <p className="text-sm px-2 py-1 border border-[#1A1F3A] rounded-full text-[#1A1F3A]">Official</p>
                    )}
                    <div className="flex gap-3">
                      <button onClick={() => {setActiveForm("link"); setSelectedLink(link); setMode("edit")}} 
                      className="text-[#6B6B5A] hover:text-green-600 transition-colors">
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M12 20h9" /><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4Z" />
                        </svg>
                        </button>
                        <button onClick={() => {setActiveForm("link"); setSelectedLink(link); setMode("delete")}}
                        className="text-[#6B6B5A] hover:text-red-600 transition-colors">
                          <svg xmlns="http://www.w3.org/2000/svg" width="18"height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="3 6 5 6 21 6" /><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                          </svg>
                          </button>
                            </div>
                    </div>
                    </div>
                    </div>
                  ))
                )}
                  <button
                  onClick={() => {
                    setActiveForm("link"); setSelectedLink(null); setMode("create")}}
                    className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-lg text-[#6B6B5A] w-full hover:bg-gray-200">
                    + Add link
                    </button>
                    </div>
                  )}
      
      {tab === "project_ideas" && (
        <div>
          {course.project_ideas.map((idea) => (
            <div key={idea.idea_id}
            className="border border-[#DAD8D6] rounded-xl p-4 mb-3 flex justify-between items-start">
              <div className="flex items-center gap-2">
                <h3 className="font-semibold text-lg text-[#1A1F3A]">{idea.title}</h3>
                <p className="text-sm px-3 py-1 rounded-full border border-[#DAD8D6] text-[#6B6B5A]">{idea.difficulty}</p>
                </div>
                <div className="flex items-center gap-2">
                {idea.official  && (
                  <p className="text-sm px-2 py-1 border border-[#1A1F3A] rounded-full text-[#1A1F3A]">Official</p>
                  )}
                  <div className="flex gap-3">
                      <button onClick={() => {setActiveForm("project_idea"); setSelectedProjectIdea(idea); setMode("edit")}} 
                      className="text-[#6B6B5A] hover:text-green-600 transition-colors">
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M12 20h9" /><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4Z" />
                        </svg>
                        </button>
                        <button onClick={() => {setActiveForm("project_idea"); setSelectedProjectIdea(idea); setMode("delete")}}
                        className="text-[#6B6B5A] hover:text-red-600 transition-colors">
                          <svg xmlns="http://www.w3.org/2000/svg" width="18"height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="3 6 5 6 21 6" /><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                          </svg>
                          </button>
                          </div>
                        </div>
                  </div>
                ))}
                <button onClick={() => {
                  setActiveForm("project_idea"); setSelectedProjectIdea(null); setMode("create")}}
                  className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-lg text-[#6B6B5A] w-full hover:bg-gray-200">+ Add project idea</button>
                </div>
              )}

      {tab === "faqs" && (
        <div>
          {course.faqs.length === 0 ? (
            <p className="text-[#6B6B5A] text-lg py-4">No faqs yet. Be the first to contribute!</p>
          ) : (
            course.faqs.map((faq) => {
              const isOpen = openFaqId === faq.faq_id;

              return (
                <div key={faq.faq_id} className="border border-[#DAD8D6] rounded-xl mb-2 overflow-hidden">
                  <div
                    onClick={() => setOpenFaqId(isOpen ? null : faq.faq_id)}
                    className={`relative flex justify-between items-center w-full text-left px-4 py-3 hover:bg-[#F4F2EB] transition-colors ${
                      isOpen
                        ? "after:absolute after:bottom-0 after:left-4 after:right-4 after:border-b after:border-[#DAD8D6]"
                        : ""
                    }`}>
                    <h3 className="font-medium text-lg text-[#1A1F3A]">{faq.question}</h3>
                    <div className="flex items-center gap-2">
                      {faq.official  && (
                        <p className="text-sm px-2 py-1 border border-[#1A1F3A] rounded-full text-[#1A1F3A]">Official</p>
                      )}
                      <div className="flex gap-3">
                      <button onClick={() => {setActiveForm("faq"); setSelectedFaq(faq); setMode("edit")}} 
                      className="text-[#6B6B5A] hover:text-green-600 transition-colors">
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M12 20h9" /><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4Z" />
                        </svg>
                        </button>
                        <button onClick={() => {setActiveForm("faq"); setSelectedFaq(faq); setMode("delete")}}
                        className="text-[#6B6B5A] hover:text-red-600 transition-colors">
                          <svg xmlns="http://www.w3.org/2000/svg" width="18"height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="3 6 5 6 21 6" /><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                          </svg>
                          </button>
                          </div>
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2"
                        className={`w-5 h-5 text-[#6B6B5A] transition-transform duration-200 ${isOpen ? "rotate-180" : ""}`}
                      >
                        <path strokeLinecap="round" strokeLinejoin="round" d="M19 9l-7 7-7-7" />
                      </svg>
                    </div>
                  </div>
                  {isOpen && (
                    <div className="px-4 pb-4 pt-1 border-t border-[#F4F2EB]">
                      <p className="text-base text-[#6B6B5A]">{faq.answer}</p>
                    </div>
                  )}
                </div>
              );
            })
          )}
          <button
            onClick={() => {setActiveForm("faq"); setSelectedFaq(null); setMode("create");}}
            className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-lg text-[#6B6B5A] w-full hover:bg-gray-200">+ Add FAQ
          </button>
        </div>
      )}
      </>
      )}
    </main>

    {showAlert && (
      <ContributeAlert onConfirm={() => {setShowAlert(false); setShowContribute(true)}} onCancel={() => setShowAlert(false)}
      />
    )}

    {showContribute && (
          <Contribute 
          courseId={id ?? ""}
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
      
    {mode && activeForm === "resource" && (
      <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/40">
        <div className="bg-white rounded-2xl p-7 max-w-lg w-full mx-4 shadow-md text-[#1A1F3A]">
          <ResourceForm
            mode={mode}
            resource={selectedResource ?? undefined}
            courseId={id ?? ""}
            onCancel={() => { setMode(null); setSelectedLink(null); setActiveForm(null);}}/>
            </div>
            </div>
          )}

    {mode && activeForm === "link" && (
      <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/40">
        <div className="bg-white rounded-2xl p-7 max-w-lg w-full mx-4 shadow-md text-[#1A1F3A]">
          <LinkForm
            mode={mode}
            link={selectedLink ?? undefined}
            courseId={id ?? ""}
            onCancel={() => { setMode(null); setSelectedLink(null); setActiveForm(null);}}/>
            </div>
            </div>
          )}

    {mode && activeForm === "project_idea" && (
      <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/40">
        <div className="bg-white rounded-2xl p-7 max-w-lg w-full mx-4 shadow-md text-[#1A1F3A]">
          <ProjectIdeaForm
            mode={mode}
            projectIdea={selectedProjectIdea ?? undefined}
            courseId={id ?? ""}
            onCancel={() => { setMode(null); setSelectedProjectIdea(null); setActiveForm(null);}}/>
            </div>
            </div>
          )}
        
    {mode && activeForm == "faq" && (
      <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/40">
        <div className="bg-white rounded-2xl p-7 max-w-lg w-full mx-4 shadow-md text-[#1A1F3A]">
          <FaqForm
            mode={mode}
            faq={selectedFaq ?? undefined}
            courseId={id ?? ""}
            onCancel={() => { setMode(null); setSelectedFaq(null); setActiveForm(null)}}/>
            </div>
            </div>
          )}
                </>
);
}