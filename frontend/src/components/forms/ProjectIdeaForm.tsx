import { useState } from "react";
import { apiUrl } from "../../config";
import { toast } from "../Toast";

interface Props {
  courseId: string;
  onCancel: () => void;
  mode?: "create" | "edit" | "delete";
  projectIdea?: { idea_id: string; title?: string; body?: string; difficulty?: "Easy" | "Medium" | "Hard"; official?: boolean };
}

type Difficulty = "Easy" | "Medium" | "Hard";

export default function ProjectIdeaForm({ courseId, onCancel, mode = "create", projectIdea }: Props) {
  const [title, setTitle] = useState(projectIdea?.title ?? "");
  const [body, setBody] = useState(projectIdea?.body ?? "");
  const [difficulty, setDifficulty] = useState<Difficulty>((projectIdea?.difficulty as Difficulty) ?? "Medium");
  const [comment, setComment] = useState("");
  const [submitted, setSubmitted] = useState(false);
  const [instantContribute, setInstantContribute] = useState(false);
  const [isOfficial, setIsOfficial] = useState(projectIdea?.official == true);

  // TODO: replace with actual auth check
  const isAdmin = true;
  const canInstantContribute = isAdmin && instantContribute;

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();

    try {
      //create project idea
      if (mode === "create") {
        const response = await fetch(
          apiUrl(canInstantContribute ? "project_idea" : "contributions"), {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            body: JSON.stringify(canInstantContribute ?
              {
                //write_text
                Create: {
                  course_id: courseId,
                  title,
                  body,
                  difficulty,
                },
              } : {
                //suggest_text
                Propose: {
                  course_id: courseId,
                  contribution: {
                    Text: {
                      AddProjectIdea: {
                        title,
                        body,
                        difficulty,
                      },
                    },
                  },
                  comment,
                },
              }
            ),
          }
        );
        // TODO: Replace this with a better error handling
        if (!response.ok) {
          const error = await response.text();
          console.error(error);
          toast(canInstantContribute ? "Failed to create project idea" : "Failed to propose project idea", false);
          return;
        }
        setSubmitted(true);
        return;
      }

      //edit project idea
      if (mode === "edit") {
        if (!projectIdea) return toast("Missing project idea id", false);

        if (canInstantContribute) {
          const response = await fetch(
            apiUrl("project_idea"), {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              credentials: "include",
              body: JSON.stringify({
                //write_text
                Update: {
                  idea_id: projectIdea.idea_id,
                  course_id: courseId,
                  title,
                  body,
                  difficulty,
                },
              }),
            }
          );
          // TODO: Replace this with a better error handling
          if (!response.ok) {
            const error = await response.text();
            console.error(error);
            toast("Failed to update project idea", false);
            return;
          }
        } else {
          const response = await fetch(
            apiUrl("contributions"), {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              credentials: "include",
              body: JSON.stringify({
                //suggest_text
                Propose: {
                  course_id: courseId,
                  contribution: {
                    Text: {
                      EditProjectIdea: {
                        idea_id: projectIdea.idea_id,
                        title,
                        body,
                        difficulty,
                      },
                    },
                  },
                  comment,
                },
              }),
            }
          );
          // TODO: Replace this with a better error handling
          if (!response.ok) {
            const error = await response.text();
            console.error(error);
            toast("Failed to propose project idea update", false);
            return;
          }
        }
        //write_text
        if (isOfficial && isAdmin) {
          const response = await fetch(
            apiUrl("project_idea"), {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              credentials: "include",
              body: JSON.stringify({
                SetOfficial: {
                  idea_id: projectIdea.idea_id,
                  course_id: courseId,
                  official: true,
                },
              }),
            }
          );
          // TODO: Replace this with a better error handling
          if (!response.ok) {
            const error = await response.text();
            console.error(error);
            toast("Failed to set project idea official", false);
            return;
          }
        }
        setSubmitted(true);
        return;
      }

      //delete project idea
      if (mode === "delete") {
        if (!projectIdea) return toast("Missing project idea id", false);

        const response = await fetch(
          apiUrl(canInstantContribute ? "project_idea" : "contributions"), {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            body: JSON.stringify(canInstantContribute ?
              {
                //write_text
                Delete: {
                  idea_id: projectIdea.idea_id,
                  course_id: courseId,
                },
              } : {
                //suggest_text
                Propose: {
                  course_id: courseId,
                  contribution: {
                    Text: {
                      RemoveProjectIdea: {
                        idea_id: projectIdea.idea_id,
                      },
                    },
                  },
                  comment,
                },
              }
            ),
          }
        );
        // TODO: Replace this with a better error handling
        if (!response.ok) {
          const error = await response.text();
          console.error(error);
          toast(canInstantContribute ? "Failed to delete project idea" : "Failed to propose project idea removal", false);
          return;
        }
        setSubmitted(true);
        return;
      }
    } catch (error) {
      console.error(error);
      toast("Something went wrong. Could not connect to server.", false);
    }
  }

  if (submitted) {
    return (
      <div className="text-center py-6 bg-[#F4F2EB] rounded-lg">
        <h3 className="text-xl font-semibold text-[#1A1F3A] mb-2">Thank you for your contribution!</h3>
        <p className="text-[#6B6B5A] mb-6">
          {canInstantContribute
            ? mode === "delete" ? "Project idea deleted successfully" : "Project idea saved successfully"
            : "Your project idea has been submitted and is waiting for review."}
        </p>
        <button type="button" onClick={onCancel} className="px-4 py-2 bg-[#1A1F3A] text-lg text-white rounded-lg hover:opacity-90">
          Back to course
        </button>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-5">
      {mode === "delete" ? (
        <div className="border rounded-lg p-4">
          Are you sure you want to remove this project idea?
          <div className="mt-4 text-sm text-[#6B6B5A]">{projectIdea?.title}</div>
        </div>
      ) : (
        <>
          <div>
            <label className="block text-lg mb-2 text-[#6B6B5A]">Project title</label>
            <input type="text" placeholder="Example: Build a Rust web server" value={title} 
            onChange={(e) => setTitle(e.target.value)} required 
            className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
          </div>
          <div>
            <label className="block text-lg mb-2 text-[#6B6B5A]">Description</label>
            <textarea placeholder="Describe the project idea..." value={body}
            onChange={(e) => setBody(e.target.value)} required rows={4}
            className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none resize-none"/>
          </div>
          <div>
            <label className="block text-lg mb-2 text-[#6B6B5A]">Difficulty</label>
            <select value={difficulty} onChange={(e) => setDifficulty(e.target.value as Difficulty)} 
            className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none">
              <option value="Easy">Easy</option>
              <option value="Medium">Medium</option>
              <option value="Hard">Hard</option>
            </select>
          </div>
        </>
      )}

      {!canInstantContribute && (
        <div>
          <label className="block text-lg mb-2 text-[#6B6B5A]">Comment</label>
          <textarea value={comment} placeholder="Anything we should know?" onChange={(e) => setComment(e.target.value)} required rows={3} className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 resize-none focus:outline-none"/>
        </div>
      )}

      {isAdmin && (
        <div>
          {mode === "edit" && (
            <label className="flex items-center gap-2 text-[#6B6B5A]">
              <input type="checkbox" checked={isOfficial} onChange={(e) => setIsOfficial(e.target.checked)}/>
              <span>Set official</span>
            </label>
          )}
          <label className="flex items-center gap-2 text-[#6B6B5A]">
            <input type="checkbox" checked={instantContribute} onChange={(e) => setInstantContribute(e.target.checked)}/>
            <span>Instant contribute</span>
          </label>
        </div>
      )}

      <div className="flex justify-end gap-3">
        <button type="button" onClick={onCancel} className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100">
          Cancel
        </button>
        <button type="submit" className="px-4 py-2 text-lg bg-[#1A1F3A] text-white rounded-lg hover:opacity-90 disabled:opacity-50">
          {mode === "delete" ? "Delete" : canInstantContribute ? "Publish" : "Send for review"}
        </button>
      </div>
    </form>
  );
}