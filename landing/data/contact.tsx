import { ReactNode } from "react";

interface ContactMethod {
  label: string;
  value: string;
  href: string;
  icon: ReactNode;
  external?: boolean;
}

interface SocialLink {
  name: string;
  href: string;
  icon: ReactNode;
}

interface ContactInfo {
  methods: ContactMethod[];
  socials: SocialLink[];
}

export const CONTACT_INFO: ContactInfo = {
  methods: [
    {
      label: "Email",
      value: "hello@atronsa.com",
      href: "mailto:hello@atronsa.com",
      icon: (
        <svg
          className="h-4 w-4 text-white/50 transition-transform duration-300 group-hover:-translate-y-0.5 group-hover:translate-x-0.5"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={1.5}
            d="M7 17L17 7M7 7h10v10"
          />
        </svg>
      ),
    },
    {
      label: "Phone",
      value: "+251 905 429 602",
      href: "tel:+251905429602",
      icon: (
        <svg
          className="h-4 w-4 text-white/50"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={1.5}
            d="M2.25 6.75c0 8.284 6.716 15 15 15h2.25a2.25 2.25 0 002.25-2.25v-1.372c0-.516-.351-.966-.852-1.09l-4.423-1.106a1.125 1.125 0 00-1.173.417l-.97 1.293c-.5.667-1.45.88-2.22.502a12.035 12.035 0 01-6.657-6.657c-.378-.77-.165-1.72.502-2.22l1.293-.97c.277-.208.39-.553.317-.89L6.51 3.884A1.125 1.125 0 005.42 3.032H4.5A2.25 2.25 0 002.25 5.282v1.468z"
          />
        </svg>
      ),
    },
    {
      label: "Phone 2",
      value: "+251 953 709 094",
      href: "tel:+251953709094",
      icon: (
        <svg
          className="h-4 w-4 text-white/50"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={1.5}
            d="M2.25 6.75c0 8.284 6.716 15 15 15h2.25a2.25 2.25 0 002.25-2.25v-1.372c0-.516-.351-.966-.852-1.09l-4.423-1.106a1.125 1.125 0 00-1.173.417l-.97 1.293c-.5.667-1.45.88-2.22.502a12.035 12.035 0 01-6.657-6.657c-.378-.77-.165-1.72.502-2.22l1.293-.97c.277-.208.39-.553.317-.89L6.51 3.884A1.125 1.125 0 005.42 3.032H4.5A2.25 2.25 0 002.25 5.282v1.468z"
          />
        </svg>
      ),
    },
    {
      label: "Telegram",
      value: "@atronsa",
      href: "https://t.me/atronsasupport",
      icon: (
        <svg
          viewBox="0 0 24 24"
          fill="currentColor"
          className="h-4 w-4 text-white/50 transition-colors group-hover:text-white"
        >
          <path d="M11.944 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0a12 12 0 0 0-.056 0zm4.962 7.224c.1-.002.321.023.465.14a.506.506 0 0 1 .171.325c.016.093.036.306.02.472-.18 1.898-.962 6.502-1.36 8.627-.168.9-.499 1.201-.82 1.23-.696.065-1.225-.46-1.9-.902-1.056-.693-1.653-1.124-2.678-1.8-1.185-.78-.417-1.21.258-1.91.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.14-5.061 3.345-.48.33-.913.49-1.302.48-.428-.008-1.252-.241-1.865-.44-.752-.245-1.349-.374-1.297-.789.027-.216.325-.437.893-.663 3.498-1.524 5.83-2.529 6.998-3.014 3.332-1.386 4.025-1.627 4.476-1.635z" />
        </svg>
      ),
      external: true,
    },
  ],
  socials: [
    {
      name: "Facebook",
      href: "https://facebook.com/atronsa",
      icon: (
        <svg viewBox="0 0 24 24" fill="currentColor" className="h-4 w-4">
          <path d="M22 12.06C22 6.5 17.52 2 12 2S2 6.5 2 12.06c0 5 3.66 9.15 8.44 9.94v-7.03H7.9v-2.91h2.54V9.85c0-2.51 1.49-3.9 3.77-3.9 1.09 0 2.24.2 2.24.2v2.46h-1.26c-1.24 0-1.63.77-1.63 1.56v1.87h2.78l-.44 2.91h-2.34V22c4.78-.79 8.44-4.94 8.44-9.94Z" />
        </svg>
      ),
    },
    {
      name: "LinkedIn",
      href: "https://linkedin.com/company/atronsa",
      icon: (
        <svg viewBox="0 0 24 24" fill="currentColor" className="h-4 w-4">
          <path d="M20.45 20.45h-3.55v-5.57c0-1.33-.02-3.03-1.85-3.03-1.85 0-2.14 1.45-2.14 2.94v5.66H9.36V9h3.41v1.56h.05c.47-.9 1.63-1.85 3.36-1.85 3.6 0 4.27 2.37 4.27 5.45v6.29ZM5.34 7.43a2.06 2.06 0 1 1 0-4.12 2.06 2.06 0 0 1 0 4.12ZM7.12 20.45H3.56V9h3.56v11.45Z" />
        </svg>
      ),
    },
    {
      name: "TikTok",
      href: "https://tiktok.com/@atronsa",
      icon: (
        <svg viewBox="0 0 24 24" fill="currentColor" className="h-4 w-4">
          <path d="M16.6 5.82c-.9-.98-1.4-2.26-1.4-3.6h-3.05v13.9a2.9 2.9 0 1 1-2.05-2.77v-3.12a6 6 0 1 0 5.1 5.93V9.4a6.8 6.8 0 0 0 4 1.28V7.6a4.4 4.4 0 0 1-2.6-1.78Z" />
        </svg>
      ),
    },
  ],
};

export const PROJECT_TYPES = [
  { value: "personal", label: "Personal Website" },
  { value: "business", label: "Company Website" },
  { value: "revamp", label: "Website Revamp" },
  { value: "other", label: "Other" },
] as const;
