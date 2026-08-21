export interface Social {
  name: string;
  href: string;
  icon: "facebook" | "linkedin" | "tiktok";
}

export const SOCIALS: Social[] = [
  { name: "Facebook", href: "https://facebook.com/atronsa", icon: "facebook" },
  {
    name: "LinkedIn",
    href: "https://linkedin.com/company/atronsa",
    icon: "linkedin",
  },
  { name: "TikTok", href: "https://tiktok.com/@atronsa", icon: "tiktok" },
];
