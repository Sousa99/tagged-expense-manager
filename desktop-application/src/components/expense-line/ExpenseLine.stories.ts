import type { Meta, StoryObj } from '@storybook/react';
import { ExpenseLine } from './ExpenseLine';

const meta = {
  title: 'Components/ExpenseLine',
  component: ExpenseLine,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    expense_title: { control: 'text' },
    expense_time: { control: 'text' },
    expense_description: { description: 'text' },
    expense_value: { control: 'number' }
  },
  args: { },
} satisfies Meta<typeof ExpenseLine>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    expense_title: 'Weekly Shopping',
    expense_time: '2024.09.09 23:11:23',
    expense_description: 'Weekly shopping for produce and products for the coming week',
    expense_value: 4500
  }
};
