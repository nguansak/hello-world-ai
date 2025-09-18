# Storybook Setup Guide

## Overview
This project now includes Storybook for component development and testing. Storybook provides an isolated environment to develop, test, and document UI components.

## Installation
Storybook has been successfully installed and configured for this React + Vite project with the following setup:

### Installed Packages
- `@storybook/react-vite` - Framework adapter for React + Vite
- `storybook` - Core Storybook package
- `eslint-plugin-storybook` - ESLint rules for Storybook
- `prop-types` - PropTypes validation

### Configuration Files
- `.storybook/main.js` - Main Storybook configuration
- `.storybook/preview.js` - Global settings and decorators (includes Tailwind CSS)

## Running Storybook

### Start Development Server
```bash
npm run storybook
```
This will start Storybook on `http://localhost:6006`

### Build Static Storybook
```bash
npm run build-storybook
```
This creates a static build in `storybook-static/` directory

## Datepicker Component Stories

The Datepicker component includes comprehensive stories showcasing various use cases:

### Available Stories
1. **Default** - Basic datepicker in a container
2. **Compact** - Minimal datepicker without extra styling
3. **WithContainer** - Styled container with header and action button
4. **DarkTheme** - Dark theme variant
5. **MultipleInstances** - Two datepickers for date range selection
6. **Responsive** - Different container sizes
7. **Playground** - Interactive testing environment

### Story Features
- **Thai Documentation** - Stories include Thai descriptions and comments
- **Responsive Testing** - Stories test different screen sizes
- **Theme Variants** - Light and dark theme examples
- **Real-world Usage** - Form integration and container examples

## Project Structure
```
├── .storybook/
│   ├── main.js          # Storybook configuration
│   └── preview.js       # Global settings
├── src/
│   └── components/
│       ├── Datepicker.jsx          # Component file
│       ├── Datepicker.stories.jsx  # Storybook stories
│       ├── Datepicker.test.jsx     # Test file
│       └── Datepicker.specs.md     # Specifications
```

## Features

### Tailwind CSS Integration
- Tailwind CSS is configured to work in Storybook
- All component styles render correctly
- Responsive utilities work as expected

### Documentation
- Automatic documentation generation with `autodocs`
- Thai language descriptions and comments
- Component specifications in `Datepicker.specs.md`

### Testing Integration
- Comprehensive test suite in `Datepicker.test.jsx`
- Visual regression testing capabilities
- Component behavior testing

## Writing New Stories

### Basic Story Structure
```jsx
import YourComponent from './YourComponent';

export default {
  title: 'Components/YourComponent',
  component: YourComponent,
  parameters: {
    layout: 'centered',
  },
};

export const Default = {
  render: () => <YourComponent />,
};
```

### Story with Args
```jsx
export const WithProps = {
  args: {
    prop1: 'value1',
    prop2: true,
  },
  render: (args) => <YourComponent {...args} />,
};
```

## Best Practices

### Story Organization
- Use descriptive story names
- Group related stories under same title
- Include documentation strings
- Test different component states

### Component Development
- Develop components in isolation
- Test various props and states
- Document usage examples
- Include accessibility considerations

### Documentation
- Add component descriptions
- Document all props
- Include usage examples
- Write comprehensive specs

## Troubleshooting

### Common Issues
1. **Tailwind styles not loading**: Ensure `import '../src/index.css'` is in `.storybook/preview.js`
2. **Component not found**: Check the stories glob pattern in `.storybook/main.js`
3. **Build errors**: Ensure all dependencies are properly installed

### Development Tips
- Use `npm run storybook` for development
- Check browser console for errors
- Use Storybook's built-in tools for testing
- Leverage the docs tab for component documentation

## Next Steps

### Potential Enhancements
1. **Add more addons**:
   - `@storybook/addon-actions` for event handling
   - `@storybook/addon-controls` for interactive props
   - `@storybook/addon-viewport` for responsive testing

2. **Visual Testing**:
   - Integrate Chromatic for visual regression testing
   - Add screenshot testing capabilities

3. **Accessibility**:
   - Add `@storybook/addon-a11y` for accessibility testing
   - Include accessibility specifications

4. **Documentation**:
   - Expand component documentation
   - Add design system documentation
   - Include usage guidelines

## Resources
- [Storybook Documentation](https://storybook.js.org/docs)
- [React + Vite Framework](https://storybook.js.org/docs/get-started/frameworks/react-vite)
- [Tailwind CSS with Storybook](https://tailwindcss.com/docs/guides/vite)
- [Writing Stories](https://storybook.js.org/docs/writing-stories)
