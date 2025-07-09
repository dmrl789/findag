import { useState, useEffect } from 'react';

export function useMediaQuery(query: string): boolean {
  const [matches, setMatches] = useState(false);

  useEffect(() => {
    const media = window.matchMedia(query);
    
    // Set initial value
    setMatches(media.matches);

    // Create event listener
    const listener = (event: MediaQueryListEvent) => {
      setMatches(event.matches);
    };

    // Add listener
    media.addEventListener('change', listener);

    // Cleanup
    return () => {
      media.removeEventListener('change', listener);
    };
  }, [query]);

  return matches;
}

// Predefined breakpoint hooks
export function useIsMobile(): boolean {
  return useMediaQuery('(max-width: 767px)');
}

export function useIsTablet(): boolean {
  return useMediaQuery('(min-width: 768px) and (max-width: 1023px)');
}

export function useIsDesktop(): boolean {
  return useMediaQuery('(min-width: 1024px)');
}

export function useIsLargeDesktop(): boolean {
  return useMediaQuery('(min-width: 1280px)');
}

// Tailwind CSS breakpoint hooks
export function useIsSm(): boolean {
  return useMediaQuery('(min-width: 640px)');
}

export function useIsMd(): boolean {
  return useMediaQuery('(min-width: 768px)');
}

export function useIsLg(): boolean {
  return useMediaQuery('(min-width: 1024px)');
}

export function useIsXl(): boolean {
  return useMediaQuery('(min-width: 1280px)');
}

export function useIs2xl(): boolean {
  return useMediaQuery('(min-width: 1536px)');
}

// Orientation hooks
export function useIsPortrait(): boolean {
  return useMediaQuery('(orientation: portrait)');
}

export function useIsLandscape(): boolean {
  return useMediaQuery('(orientation: landscape)');
}

// Feature detection hooks
export function useSupportsHover(): boolean {
  return useMediaQuery('(hover: hover)');
}

export function useSupportsTouch(): boolean {
  return useMediaQuery('(pointer: coarse)');
}

export function usePrefersReducedMotion(): boolean {
  return useMediaQuery('(prefers-reduced-motion: reduce)');
}

export function usePrefersDarkMode(): boolean {
  return useMediaQuery('(prefers-color-scheme: dark)');
} 