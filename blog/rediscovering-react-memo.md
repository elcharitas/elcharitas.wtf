---
title: "Rediscovering React.memo 🤯"
slug: rediscovering-react-memo
tags: react

---

React is a popular and powerful JavaScript library for building user interfaces. However, as applications grow and become more complex, performance can become an issue. One effective way to optimize the performance of React applications is to use the `React.memo` higher-order component, which allows components to skip rendering when their props don't change.

To understand how `React.memo` works and how it can improve performance, it's important to first understand the basic principles of React's rendering process. When a React application is rendered, the virtual DOM (Document Object Model) is updated to reflect the latest state of the application. This process can be computationally expensive, especially for larger and more complex applications.

To optimize this process, React uses a concept called reconciliation, which compares the virtual DOM with the real DOM and determines the minimum number of updates that need to be made. This helps to minimize the amount of work that the browser has to do and can improve the performance of the application.

One way to further optimize this process is to use `React.memo`. This is a higher-order component that wraps a component and provides a performance optimization. It works by only re-rendering the wrapped component if its props have changed. If the props are the same as the previous render, the wrapped component is not re-rendered. This can help to reduce the number of unnecessary re-renders and improve the overall performance of the application.

To use `React.memo`, you simply wrap the component that you want to optimize in the `React.memo` function. For example:

```javascript
const MyComponent = React.memo(function MyComponent(props) {
  // Component implementation
});
```

By default, `React.memo` will compare the props of the wrapped component using a shallow equality check. This means that it will only re-render the component if the props are different from the previous render in a shallow sense. This can be sufficient for many cases, but there are some situations where you may want to provide your comparison function to `React.memo`.

For example, if your component has props that are complex objects or arrays, the default shallow comparison may not be sufficient. In these cases, you can provide a custom comparison function to `React.memo` that specifies how the props should be compared. This function should take two sets of props as arguments and return `true` if the props are the same and `false` if they are different.

Here is an example of how you might use a custom comparison function with `React.memo`:

```javascript
const areEqual = (prevProps, nextProps) => {
  // Custom comparison logic here
  return true; // or false
};

const MyComponent = React.memo(function MyComponent(props) {
  // Component implementation
}, areEqual);
```

In conclusion, memos are a great way to optimize your react application. The benefits are just so much 😇.
