# Superpowered JavaScript Pure Functions 😇

In JavaScript, a function is a block of code that is designed to perform a specific task. It can take input in the form of arguments and can return a value as output. Functions are an essential part of JavaScript, and they are used to make the code more modular and reusable.

One type of function that is particularly useful in JavaScript is a pure function. A pure function is a function that has several important characteristics:

1.  It always returns the same output for a given input.
    
2.  It does not have any side effects, meaning it does not modify any external state.
    
3.  It does not depend on any external state, meaning it only depends on its input arguments.
    

These characteristics make pure functions easy to understand, test, and reason about. They also make it possible to optimize and parallelize the code that uses pure functions, because the behavior of a pure function is predictable and does not depend on any external factors.

In this article, we will explore the concept of pure functions in more detail, and we will see how they can be used to write powerful and efficient code in JavaScript.

**What is a Pure Function?**

A pure function is a function that satisfies the three conditions mentioned earlier: it always returns the same output for a given input, it does not have any side effects, and it does not depend on any external state. Let's take a closer look at each of these conditions.

The first condition states that a pure function always returns the same output for a given input. This means that if you call the same pure function multiple times with the same arguments, it will always return the same result. For example, consider the following pure function:

```javascript
function add(x, y) {
  return x + y;
}
```

This function takes two arguments, `x` and `y`, and it returns their sum. Because it does not depend on any external state, and it does not have any side effects, it is a pure function. If we call this function with the same arguments multiple times, it will always return the same result:

```javascript
add(1, 2); // returns 3
add(1, 2); // still returns 3
add(1, 2); // always returning 3 🌚
```

The second condition states that a pure function does not have any side effects. This means that it does not modify ***any external state***, such as global variables or the properties of objects. For example, consider the following impure function:

```javascript
let counter = 0;

function incrementCounter() {
  counter++;
}
```

This function increments a global variable, `counter`, by 1 each time it is called. Because it modifies an external state, it is an impure function. If we call this function multiple times, the value of `counter` will be different each time:

```javascript
incrementCounter(); // counter is now 1
incrementCounter(); // counter is now 2
incrementCounter(); // counter is now 3
```

The third condition states that a pure function does not depend on any external state. This means that it only depends on its input arguments, and it does not rely on any global variables or the properties of objects. For example, consider the following impure function:

```javascript
let user = {
  name: "John Doe",
  age: 30,
};

function greetUser() {
  console.log(`Hello, ${user.name}!`);
}
```

This function prints a different greeting based on the name of the external `user` object. A pure function should never have to depend on an external state for it to `function`. And this is why pure functions are pretty much superpowered. They're isolated from external states making them easily debuggable.

In conclusion, pure functions are really powerful and can be utilized in ways that improve overall DX. They can be created and consumed in parallel since they're isolated from external states. Hoping to see more pure functions out there saving the developer world.