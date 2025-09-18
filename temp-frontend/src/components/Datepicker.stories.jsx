import Datepicker from './Datepicker';

/**
 * มาตรฐาน Meta configuration สำหรับ Datepicker component
 * ใช้สำหรับ configure การแสดงผลใน Storybook
 */
export default {
  title: 'Components/Datepicker',
  component: Datepicker,
  parameters: {
    layout: 'centered', // จัด component ให้อยู่กลางจอ
    docs: {
      description: {
        component: 'Datepicker component สำหรับเลือกวันที่ พร้อมด้วย UI แบบ calendar grid ที่ใช้ Tailwind CSS styling',
      },
    },
  },
  tags: ['autodocs'], // เปิดใช้ automatic documentation
  argTypes: {
    // สำหรับ future development อาจเพิ่ม props เช่น:
    // initialDate: { control: 'date' },
    // onDateSelect: { action: 'date-selected' },
    // disabled: { control: 'boolean' },
    // minDate: { control: 'date' },
    // maxDate: { control: 'date' },
  },
};

/**
 * Default story - แสดง Datepicker ในสถานะปกติ
 */
export const Default = {
  render: () => (
    <div className="p-4 border border-gray-200 rounded-lg shadow-sm">
      <h3 className="text-lg font-semibold mb-4 text-gray-800">Select Date</h3>
      <Datepicker />
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Datepicker component ในรูปแบบ default แสดงเดือนปัจจุบันและให้เลือกวันที่ได้',
      },
    },
  },
};

/**
 * Compact story - แสดง Datepicker แบบกะทัดรัด
 */
export const Compact = {
  render: () => (
    <div className="inline-block">
      <Datepicker />
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Datepicker component แบบกะทัดรัด เหมาะสำหรับใช้ใน form หรือ modal',
      },
    },
  },
};

/**
 * With Container story - แสดง Datepicker ภายใน container ที่มี styling
 */
export const WithContainer = {
  render: () => (
    <div className="max-w-sm mx-auto">
      <div className="bg-white p-6 rounded-xl shadow-lg border border-gray-100">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold text-gray-900">เลือกวันที่</h2>
          <div className="w-2 h-2 bg-purple-600 rounded-full"></div>
        </div>
        <Datepicker />
        <div className="mt-4 pt-4 border-t border-gray-100">
          <button className="w-full bg-purple-600 text-white py-2 px-4 rounded-lg hover:bg-purple-700 transition-colors">
            ยืนยัน
          </button>
        </div>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Datepicker component ที่ใช้ภายใน container พร้อม header และ action button',
      },
    },
  },
};

/**
 * Dark Theme story - แสดง Datepicker ใน dark theme
 */
export const DarkTheme = {
  render: () => (
    <div className="bg-gray-900 p-6 rounded-lg">
      <h3 className="text-lg font-semibold mb-4 text-white">Select Date</h3>
      <div className="bg-gray-800 p-4 rounded-lg">
        <Datepicker />
      </div>
    </div>
  ),
  parameters: {
    backgrounds: {
      default: 'dark',
      values: [
        { name: 'dark', value: '#1f2937' },
      ],
    },
    docs: {
      description: {
        story: 'Datepicker component ใน dark theme environment',
      },
    },
  },
};

/**
 * Multiple Instances story - แสดง Datepicker หลายตัว
 */
export const MultipleInstances = {
  render: () => (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-6 p-4">
      <div className="space-y-3">
        <h4 className="font-semibold text-gray-700">From Date</h4>
        <div className="border border-gray-200 p-3 rounded-lg">
          <Datepicker />
        </div>
      </div>
      <div className="space-y-3">
        <h4 className="font-semibold text-gray-700">To Date</h4>
        <div className="border border-gray-200 p-3 rounded-lg">
          <Datepicker />
        </div>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'การใช้ Datepicker หลายตัวในหน้าเดียวกัน เช่น สำหรับเลือกช่วงวันที่',
      },
    },
  },
};

/**
 * Responsive story - แสดง Datepicker ในหลายขนาดหน้าจอ
 */
export const Responsive = {
  render: () => (
    <div className="space-y-6">
      {/* Mobile view */}
      <div className="w-64">
        <h5 className="text-sm font-medium text-gray-600 mb-2">Mobile View (256px)</h5>
        <div className="border border-gray-200 p-3 rounded-lg">
          <Datepicker />
        </div>
      </div>
      
      {/* Tablet view */}
      <div className="w-80">
        <h5 className="text-sm font-medium text-gray-600 mb-2">Tablet View (320px)</h5>
        <div className="border border-gray-200 p-4 rounded-lg">
          <Datepicker />
        </div>
      </div>
      
      {/* Desktop view */}
      <div className="w-96">
        <h5 className="text-sm font-medium text-gray-600 mb-2">Desktop View (384px)</h5>
        <div className="border border-gray-200 p-6 rounded-lg">
          <Datepicker />
        </div>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Datepicker component ในขนาดหน้าจอต่างๆ เพื่อทดสอบ responsive design',
      },
    },
  },
};

/**
 * Playground story - สำหรับทดสอบและ experiment
 */
export const Playground = {
  render: () => (
    <div className="max-w-md mx-auto p-6">
      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          วันที่
        </label>
        <div className="border border-gray-300 rounded-lg p-3 bg-gray-50">
          <Datepicker />
        </div>
      </div>
      <div className="text-xs text-gray-500 mt-2">
        * นี่เป็น playground สำหรับทดสอบ component
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Playground story สำหรับทดสอบและ experiment กับ Datepicker component',
      },
    },
  },
};
