---
title: "Announcing Chakra UI Svelte"
slug: announcing-chakra-ui-svelte
tags: open-source

---

## Introduction
I'm so excited to share what I spent the better part of April building - Chakra UI implementation in svelte. This is an implementation of the popular [Chakra UI](https://chakra-ui.com) just as we have for [Vue](https://vue.chakra-ui.com). I'm currently working on proper documentation for it. However, the package follows the Chakra UI React style guide.

## What is Chakra UI?
Chakra UI is a simple, modular and accessible component library that gives you the building blocks you need to build your React applications. I fell in love with Chakra UI last year when I first tried the framework. Being more of a svelte lover, I was looking for [implementation in svelte](https://github.com/chakra-ui/chakra-ui/issues/819) but didn't find any. So, I set out to create it.

It took me a whole month of testing components and checking style practices. I was also looking into [Chakra UI React source](https://github.com/chakra-ui/chakra-ui). I wanted the implementation to be so closely related to React. And at the moment, I've completed about 13 components.

## Release Info
- Current Stable Prerelease Version: 0.0.2-next.2
- [Github](https://github.com/elcharitas/chakra-ui-svelte)
- [NPM](https://www.npmjs.com/package/chakra-ui-svelte)
- [Demo Website](https://chakra-svelte.vercel.app)

## Getting Started
To get started, simply install the package with
```sh
pnpm I chakra-ui-svelte
```
You may optionally install the svelte-icons package. It's the only icon package I've tested so far with chakra-ui-svelte. It's fairly stable and I have the plan of fully supporting it later on.
```sh
pnpm I svelte-icons
```

### Sample Application
Let's try building a sample app, shall we?
First, we need to import the ChakraProvider component which handles themes and other global configurations.
```
import { ChakraProvider } from 'chakra-ui-svelte';
```
Now we can go on ahead and use the provider to handle some of the stores which chakra-ui-svelte provides by default. and import other packages
```html
<script>
  import { Box, Button, ChakraProvider } from 'chakra-ui-svelte';
</script>

<ChakraProvider>
   <Box>Hello Box</Box>
   <Button> Hello Button </Button>
</ChakraProvider>
```
### Styling Components
We can also customize our components just like we do  in Chakra Ui React.
```html
<script>
  import { Button, Icon } from 'chakra-ui-svelte';
  import FaGithub from "svelte-icons/fa/FaGithub.svelte";
</script>
<Button
    as="a"
    p="0"
    mx="3"
    size="xs"
    bg="transparent"
    href={"https://github.com/elcharitas/chakra-ui-svelte"}
    target="_blank"
    rel="noreferrer"
    ariaLabel="Chakra UI on Github"
    variant="ghost"
>
    <Icon as={FaGithub} />
</Button>
```

## Final  Thoughts?
Sadly, there isn't any proper documentation at the time of this writing. However, a demo is available [here](https://chakra-svelte.vercel.app). I'd be updating the page with documentation in the coming week.
Please leave feedback and feel free to contribute to the package.
