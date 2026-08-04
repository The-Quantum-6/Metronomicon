import { jwtDecode } from 'jwt-decode';
import { useEffect, useState } from 'react';

function getCookie(name: string): string | null {
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${name}=`);
  if (parts.length === 2) {
    return parts.pop()?.split(';').shift() || null;
  }
  return null;
}

interface PermsPayload {
  sub: string;
  resource_id: string;
  perms: number;
  exp: number;
  role: string;
}



export function getPermsFromCookie(course_id: string): number {
  const token = getCookie(course_id);

  if (!token) {
    return 0;
  }

  try {
    const decoded = jwtDecode<PermsPayload>(token);
    return decoded.perms;
  } catch (error) {
    return 0;
  }
}

export function GetRoleFromCookie(course_id: string): string | null {
  const token = getCookie(course_id);

  if (!token) {
    return null;
  }

  try {
    const decoded = jwtDecode<PermsPayload>(token);
    return decoded.role;
  } catch (error) {
    return null;
  }
}

export function useCoursePerms(courseId: string | undefined) {
  const [perms, setPerms] = useState<number>(0);

  useEffect(() => {
    if (!courseId) return;
    const p = getPermsFromCookie(courseId);
    setPerms(p);
  }, [courseId]);


  const hasPerm = (requiredBit: number): boolean => {
    return hasPermission(perms, requiredBit);
  };

  return { perms, hasPerm };
}
export const PERMISSIONS = {
  READ: 1 << 1,
  WRITE_TEXT: 1 << 2,
  WRITE_FILE: 1 << 3,
  SUGGEST_TEXT: 1 << 4,
  SUGGEST_FILE: 1 << 5,
  MODERATE_TEXT: 1 << 6,
  MODERATE_FILE: 1 << 7,
  PAGE_ADMIN: 1 << 8,
  TRANSFER_PERMS: 1 << 9,
} as const;

export function hasPermission(perms: number, flag: number): boolean {
  return (perms & flag) === flag;
}