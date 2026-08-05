import { useState } from "react";
import { apiUrl } from "../../config";
import { toast } from "../Toast";

interface Props {
  courseId: string;
  onCancel: () => void;
  mode?: "create" | "edit" | "delete";
  resource?: { resource_id: string; title?: string; key?: string; official?: boolean };
}

// TODO: use courseId + file when upload endpoint is ready
export default function ResourceForm({ courseId, onCancel, mode = "create", resource }: Props) {
  const [title, setTitle] = useState(resource?.title ?? "");
  const [file, setFile] = useState<File | null>(null);
  const [comment, setComment] = useState("");
  const [submitted, setSubmitted] = useState(false);
  const [instantContribute, setInstantContribute] = useState(false);
  const [isOfficial, setIsOfficial] = useState(resource?.official == true);

  // TODO: replace with actual auth check
  const isAdmin = true;
  const canInstantContribute = isAdmin && instantContribute;

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();

    try {
      //create resource
      if (mode === "create") {
        // TODO: switch to multipart/form-data once file upload is wired up
        const response = await fetch(
          apiUrl(canInstantContribute ? "resources" : "contributions"), {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            body: JSON.stringify(canInstantContribute ?
              {
                //write_text
                Create: {
                  course_id: courseId,
                  title,
                },
              } : {
                //suggest_text
                Propose: {
                  course_id: courseId,
                  contribution: {
                    Text: {
                      AddResource: {
                        title,
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
          alert(canInstantContribute ? "Failed to create resource" : "Failed to propose resource");
          return;
        }
        setSubmitted(true);
        return;
      }

      //edit resource
      if (mode === "edit") {
        if (canInstantContribute) {
          const response = await fetch(
            apiUrl("resources"), {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              credentials: "include",
              body: JSON.stringify({
                //write_text
                Update: {
                  resource_id: resource?.resource_id,
                  course_id: courseId,
                  title,
                },
              }),
            }
          );
          // TODO: Replace this with a better error handling
          if (!response.ok) {
            const error = await response.text();
            console.error(error);
            alert("Failed to update resource");
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
                      EditResource: {
                        resource_id: resource?.resource_id,
                        title,
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
            alert("Failed to propose resource update");
            return;
          }
        }

        //write_text
        if (isOfficial && isAdmin) {
          const response = await fetch(
            apiUrl("resources"), {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              credentials: "include",
              body: JSON.stringify({
                SetOfficial: {
                  resource_id: resource?.resource_id,
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
            alert("Failed to set resource official");
            return;
          }
        }

        setSubmitted(true);
        return;
      }

      //delete resource
      if (mode === "delete") {
        const response = await fetch(
          apiUrl(canInstantContribute ? "resources" : "contributions"), {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            body: JSON.stringify(canInstantContribute ?
              {
                //write_text
                Delete: {
                  resource_id: resource?.resource_id,
                  course_id: courseId,
                },
              } : {
                //suggest_text
                Propose: {
                  course_id: courseId,
                  contribution: {
                    Text: {
                      RemoveResource: {
                        resource_id: resource?.resource_id,
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
          alert(canInstantContribute ? "Failed to delete resource" : "Failed to propose resource removal");
          return;
        }
        setSubmitted(true);
        return;
      }
    } catch (error) {
      console.error(error);
      alert("Something went wrong. Could not connect to server.");
    }
  }

  if (submitted) {
    return (
      <div className="text-center py-6 bg-[#F4F2EB] rounded-lg">
        <h3 className="text-xl font-semibold text-[#1A1F3A] mb-2">Thank you for your contribution!</h3>
        <p className="text-[#6B6B5A] mb-6">
          {canInstantContribute
            ? mode === "delete" ? "Resource deleted successfully" : "Resource saved successfully"
            : "Your request has been submitted for review."}
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
          Are you sure you want to remove this resource?
          <div className="mt-4 text-sm text-[#6B6B5A]">{resource?.title}</div>
        </div>
      ) : mode === "edit" ? (
        <div>
          <label className="block text-lg mb-2 text-[#6B6B5A]">Title</label>
          <input type="text" placeholder="Example: Lecture notes week 1" value={title}
            onChange={(e) => setTitle(e.target.value)} required
            className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
        </div>
      ) : (
        <>
          <div>
            <label className="block text-lg mb-2 text-[#6B6B5A]">Title</label>
            <input type="text" placeholder="Example: Lecture notes week 1" value={title}
              onChange={(e) => setTitle(e.target.value)} required
              className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
          </div>
          <div>
            <label className="block text-lg mb-2 text-[#6B6B5A]">File</label>
            {file ? (
              <div className="flex items-center justify-center border border-[#6B6B5A] rounded-lg px-4 py-3">
                <div className="flex items-center w-full">
                  <p className="text-lg text-[#6B6B5A]">{file.name}</p>
                  <button type="button" onClick={() => setFile(null)}
                    className="ml-auto text-[#6B6B5A] hover:text-red-600 transition-colors">
                    Remove
                  </button>
                </div>
              </div>
            ) : (
              <label className="flex flex-col items-center justify-center w-full border-2 border-dashed border-[#6B6B5A] rounded-lg py-8 cursor-pointer hover:bg-gray-50 transition-colors">
                <svg className="w-8 h-8 text-[#6B6B5A] mb-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M12 19V5m0 0-4 4m4-4 4 4M5 19h14"/>
                </svg>
                <p className="text-[#6B6B5A] text-base">Click to upload a file</p>
                <input type="file" onChange={(e) => setFile(e.target.files?.[0] ?? null)} required className="hidden"/>
              </label>
            )}
          </div>
        </>
      )}

      {!canInstantContribute && mode !== "delete" && (
        <div>
          <label className="block text-lg mb-2 text-[#6B6B5A]">Comment</label>
          <textarea placeholder="Anything reviewers should know..." value={comment}
            onChange={(e) => setComment(e.target.value)} required rows={3}
            className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 resize-none focus:outline-none"/>
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

      <div className="flex justify-end gap-3 pt-2">
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