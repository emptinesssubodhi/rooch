'use client';

export function isMainNetwork() {
  return true;
  if (typeof window !== 'undefined') {
    return (
      window.location.hostname === 'portal.rooch.network' ||
      window.location.hostname === 'main-portal.rooch.network'
    );
  }
  return false;
}
