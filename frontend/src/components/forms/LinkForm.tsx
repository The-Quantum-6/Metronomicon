import { useState } from "react";
import { apiUrl } from "../../config";
import type { Link } from "../../types/courseTypes";
import { toast } from "../Toast";

interface Props {
  mode: "create" | "edit" | "delete";
  courseId: string;
  link?: Link;
  onCancel: () => void;
}

export default function LinkForm({ courseId, onCancel, mode = "create", link }: Props) {
  const [label, setLabel] = useState(link?.label ?? "");
  const [url, setUrl] = useState(link?.url ?? "");
  const [comment, setComment] = useState("");
  const [submitted, setSubmitted] = useState(false);
  const [instantContribute, setInstantContribute] = useState(false);
  const [isOfficial, setIsOfficial] = useState(link?.official == true);

  // TODO: replace with actual auth check
  const isAdmin = true;
  const canInstantContribute = isAdmin && instantContribute;

  async function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault();

    try {
      //create link
      if (mode === "create") {
        const response = await fetch(
          apiUrl(canInstantContribute ? "links" : "contributions"), {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            body: JSON.stringify(canInstantContribute ?
              {
                //write_text
                Create: {
                  course_id: courseId,
                  label,
                  url,
                },
              } : {
                //suggest_text
                Propose: {
                  course_id: courseId,
                  contribution: {
                    Text: {
                      AddLink: {
                        label,
                        url,
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
          toast(canInstantContribute ? "Failed to create link" : "Failed to propose link", false);
          return;
        }
        setSubmitted(true);
        return;
      }

      //edit link
      if (mode === "edit") {
        if (canInstantContribute) {
          const response = await fetch(
            apiUrl("links"), {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              credentials: "include",
              body: JSON.stringify({
                //write_text
                Update: {
                  link_id: link?.link_id,
                  course_id: courseId,
                  label,
                  url,
                },
              }),
            }
          );
          // TODO: Replace this with a better error handling
          if (!response.ok) {
            const error = await response.text();
            console.error(error);
            toast("Failed to update link", false);
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
                      EditLink: {
                        link_id: link?.link_id,
                        label,
                        url,
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
            toast("Failed to propose link update", false);
            return;
          }
        }
        //write_text
        if (isOfficial && isAdmin) {
          const response = await fetch(
            apiUrl("links"), {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              credentials: "include",
              body: JSON.stringify({
                SetOfficial: {
                  link_id: link?.link_id,
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
            toast("Failed to set link official", false);
            return;
          }
        }
        setSubmitted(true);
        return;
      }

      //delete link
      if (mode === "delete") {
        const response = await fetch(
          apiUrl(canInstantContribute ? "links" : "contributions"), {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            body: JSON.stringify(canInstantContribute ?
              {
                //write_text
                Delete: {
                  link_id: link?.link_id,
                  course_id: courseId,
                },
              } : {
                //suggest_text
                Propose: {
                  course_id: courseId,
                  contribution: {
                    Text: {
                      RemoveLink: {
                        link_id: link?.link_id,
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
          toast(canInstantContribute ? "Failed to delete link" : "Failed to propose link removal", false);
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
            ? mode === "delete" ? "Link deleted successfully" : "Link saved successfully"
            : "Your request has been submitted for review."}
        </p>
        <button type="button" onClick={onCancel} className="px-4 py-2 bg-[#1A1F3A] text-lg text-white rounded-lg hover:opacity-90 transition-colors">
          Back to course
        </button>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-5">
      {mode === "delete" ? (
        <div className="border rounded-lg p-4">
          Are you sure you want to remove this link?
          <div className="mt-4 text-sm text-[#6B6B5A]">{link?.label}</div>
        </div>
      ) : (
        <>
          <div>
            <label className="block text-lg mb-2 text-[#6B6B5A]">Label</label>
            <input type="text" placeholder="Example: Understand Git better..." value={label} 
            onChange={(e) => setLabel(e.target.value)} required 
            className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
          </div>
          <div>
            <label className="block text-lg mb-2 text-[#6B6B5A]">URL</label>
            <input type="url" placeholder="https://..." value={url}
            onChange={(e) => setUrl(e.target.value)} required
            className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
          </div>
        </>
      )}

      {!canInstantContribute && (
        <div>
          <label className="block text-lg mb-2 text-[#6B6B5A]">Comment</label>
          <textarea value={comment} placeholder="Anything reviewers should know..."
          onChange={(e) => setComment(e.target.value)} required rows={3}
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 resize-none focus:outline-none"/>
        </div>
      )}

      {isAdmin && (
        <div>
          {mode === "edit" && (
            <label className="flex items-center gap-2 text-[#6B6B5A]">
              <input type="checkbox" checked={isOfficial} 
              onChange={(e) => setIsOfficial(e.target.checked)}/>
              <span>Set official</span>
            </label>
          )}
          <label className="flex items-center gap-2 text-[#6B6B5A]">
            <input type="checkbox" checked={instantContribute}
            onChange={(e) => setInstantContribute(e.target.checked)}/>
            <span>Instant contribute</span>
          </label>
        </div>
      )}

      <div className="flex justify-end gap-3">
        <button type="button" onClick={onCancel} className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100 transition-colors">
          Cancel
        </button>
        <button type="submit" className="px-4 py-2 text-lg bg-[#1A1F3A] text-white rounded-lg hover:opacity-90 transition-colors disabled:opacity-50">
          {mode === "delete" ? "Delete" : canInstantContribute ? "Publish" : "Send for review"}
        </button>
      </div>
    </form>
  );
}