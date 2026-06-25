---
title: "The start of a new adventure"
datePublished: Mon Sep 25 2023 12:16:03 GMT+0000 (Coordinated Universal Time)
cuid: clmyur5s500010aif1vwyheci
slug: the-start-of-a-new-adventure
cover: https://cdn.hashnode.com/res/hashnode/image/upload/v1695587203413/1175b487-5e84-4874-80c4-eb99a8be8750.png
tags: blog, programming-blogs, portfolio

---

I'll start with "Aloha" 😅. It's been a long while since I put something out here and this is primarily because my primary domain "elcharitas.dev" is no longer available. Sometimes late last year, I got robbed 😭, lost access to accounts backed up via my mobile and while I managed to regain access to most of my accounts, the case was different for my registry account as it couldn't be retrieved. This meant I couldn't renew my domain subscription and well... It got backordered.

After losing access to my account, the first steps I took was ensuring all tied services were removed and zero access granted to the mail concerned. I felt bad losing my domain. However, this wouldn't stop me from starting a new adventure.

So about six weeks ago, I started planning out how the new portfolio would be and today, it's live and ready for use at https://elcharitas.wtf. This write-up would give insights into what I learned, why I made certain decisions and what's next from here. Without further ado, let's dive in.

### **The Site:** [**elcharitas.wtf**](https://elcharitas.wtf)

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1695588611721/bd0b9372-6c64-4e5e-b351-a6cf8dd1d072.png)

A few things before we jump in, I would love to say a big shout-out to Andreas who made his portfolio available - [https://github.com/chronark/chronark.com](https://github.com/chronark/chronark.com). If you're familiar with [https://chronark.com/](https://chronark.com/), you'll realize my portfolio is based on his. His great work allowed me to build what's necessary and only modify the UI to suit my desires. And so, I created three basic sections on my portfolio which I will explain shortly and these are:

* Blog
    
* Projects
    
* Adventures
    

### Blogging Made Simple with Hashnode

For those unaware, my blog has always been hosted on [https://hashnode.com/](https://hashnode.com/). And thanks to hashnode's API, I was previously able to display my blog posts on my previous portfolio website which you can find the source code at [https://github.com/elcharitas/elcharitas.dev](https://github.com/elcharitas/elcharitas.dev)

This formed a foundation for how the new one would not only retrieve the posts but also render the content. There's still quite a lot I've not figured out, for instance, commenting or reacting. All of which I would figure out with time I'm sure.

Here's a preview of how the blog listing looks like:

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1695642102787/1ef8a88f-6e29-4000-b883-3a7825eafd1f.png)

At this point, I should give some big shoutouts to my friends who helped in reviewing the design, look, feel and experience of the blog section in particular.

### Github Projects all the way

I had always envisioned the possibility of managing and displaying my projects on my portfolio for a long while. One of my first approaches to this is the use of `git submodules` which didn't work out as expected.

I had earlier attempted to use [https://github.com/elcharitas/elcharitas](https://github.com/elcharitas/elcharitas) as a starting point, a sort of repo that houses all projects. The problem with this is pretty straightforward, there would need to always update the submodules once the main repo gets an update as well and while I wrote a workflow that does something similar, it felt like an overkill. If you're interested, you can check out the workflow directly at [https://github.com/elcharitas/elcharitas/blob/master/.github/workflows/update\_modules.yml](https://github.com/elcharitas/elcharitas/blob/master/.github/workflows/update_modules.yml)

So I kept researching, and hours of research aided me in consuming GitHub API and not just this, I decided on a few key constraints since I had contributed to over 200 GitHub repositories and I'm a collaborator or owner on 350+ repositories, it just wouldn't make any sense to show them all. The key constraints I decided on were:

1. A project is a public repository
    
2. A project must be one I created myself or got added to
    
3. A project must be of type "source" which means I created it myself or I got added to it.
    
4. A project must have one stargazer this helps me validate it acceptance.
    
5. Projects would be sorted by stars and not by date.
    

I decided on these constraints to allow me not only to validate the acceptance of the project but to also showcase growth and love for the project.

Here's a preview of projects:

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1695643297863/0fb6a55c-7fec-4ed8-81ae-c744558f49bd.png)

### A Timeline for Everything Tech

Finally, the last section is **adventures**. The idea for adventures came from LordGhostX's timeline - [https://lordghostx.github.io/](https://lordghostx.github.io/) It fascinated me how easy it is to not only keep track but also document the journey so I decided to build something similar. As of the time of this writing, there's still so much missing from adventures which I would add with time.

The key thing about adventures is to keep track. I had to go over my old profiles and notes to double-check and verify each piece of information as well. Coder's rank also helped a lot. You may want to check my profile [https://profile.codersrank.io/user/elcharitas](https://profile.codersrank.io/user/elcharitas)

Here's a preview of adventures

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1695644216092/88f949f2-5232-49f6-8fea-66ddf3fe602d.png)

That's it, folks. In case you've not given it a spin yet, you should. My newest and probably going to be the most stable for a long while portfolio is now live at https://elcharitas.wtf. Please note that I wrote this on a whim and would probably not edit it until I get the chance.
