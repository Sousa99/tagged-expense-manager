import type { Preview } from "@storybook/react";

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
    backgrounds: {
      default: "tem-background",
      values: [
        {
          name: "tem-background",
          value: "#0093E9 linear-gradient(160deg, #0093E9 0%, #80D0C7 100%)",
        },
      ],
    },
  },
};

export default preview;
