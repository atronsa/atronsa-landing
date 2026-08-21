interface Stat {
  value: string;
  label: string;
  rotate: string;
  position: string;
}

export const STATS: Stat[] = [
  {
    value: "200%",
    label: "Faster Performance",
    rotate: "rotate-[8deg]",
    position: "top-[60px] right-[20px]",
  },
  {
    value: "50K+",
    label: "Users Reached",
    rotate: "rotate-[-6deg]",
    position: "top-[200px] right-[120px]",
  },
  {
    value: "99.9%",
    label: "Uptime",
    rotate: "rotate-[10deg]",
    position: "top-[340px] right-[40px]",
  },
  {
    value: "2×",
    label: "More Engagement",
    rotate: "rotate-[-8deg]",
    position: "top-[480px] right-[140px]",
  },
];
