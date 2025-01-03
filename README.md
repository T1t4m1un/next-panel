# Next Panel

这是一个为NAS场景开发的面板应用，`Next Panel`目标是在[hslr-s/sun-panel](https://github.com/hslr-s/sun-panel)上提供更好、更多、更可用的功能，真正达到具备`浏览器首页/多功能平台`级别的能力。

**更好的功能：**

- 更好的**断网/慢网表现**：对于使用`CF Tunnel`等境外或个人`STUN`进行内网穿透的应用而言，`sun-panel`首屏加载速度过慢，难以负担`浏览器首页服务`的要求。`Next Panel`致力真正达到`浏览器首页`级别的服务可用性，在无网络情况下仍然以降级形式提供首页服务。

- 更好的**内外网支持**：`sun-panel`支持对内外网URL进行分别的配置，但无论在内网或外网访问下都会默认使用外网模式和外网URL提供应用打开。`Next Panel`将会提供更好的内外网切换体验，哪怕使用外网访问，只要内网服务可以访问，则自动使用内网URL打开应用。此外，`Next Panel`还会支持单内网或外网的应用。

- 更好的**多用户**：`sun-panel`的用户之间无法共享同一套应用列表，需要从零进行配置。然而，对于NAS场景，我们基本希望NAS上的应用能够对允许的用户（或角色）开放。`Next Panel`将提供更自由的方式兼顾多用户的隔离性和单NAS的应用配置重复性，不再需要每个新用户都需要重复配置应用列表。

**更多的功能：**

- 创新的**应用发现**：继续对`sun-panel`的应用配置发难。既然`sun-panel`已经有能力对`docker.socks`进行访问了，为什么不能直接从`docker`查询到需要提供支持的应用呢？`Next Panel`除了常规的配置手段外，允许通过在容器上配置`container label`，实现自动服务发现。让在NAS上添加应用最多添加一次（配置`dockere-compose.yml`）。

- 创新的**网关**：谈完应用发现，让我们再谈谈网关。对于`CF Tunnel`等服务来说，由于主机并不向公网暴露任何端口，便无需担心来自公网的直接威胁。但对于使用公网IP的服务而言，向公网暴露过多的接口来提供服务是相当危险的。`Next Panel`不仅是一个面板，还是一个网关——只需要暴露`Next Panel`一个服务，`Next Panel`会将往来流量转发到它需要去的地方。

- 创新的**推送服务**：`sun-panel`对标`浏览器首页`进行设计，这很好，NAS就需要这个——但是能不能更多呢？对于在NAS上部署自己编写的应用的用户而言，如何将NAS的信息及时推送到终端设备上是一个痛点，常用的方式是挂一个IM机器人或使用邮件通知，可为什么不直接交给`Next Panel`呢？

- 创新的**小组件**：`sun-panel`提供了简单的组件功能，可以添加有限的组件。NAS用户人均码力充沛，在源码基础上扩展组件功能也不困难——但为什么不直接允许插件呢？

**更可用的功能：**

- 可以**直接迁移**：迁移任何大小的系统都是麻烦的，`Next Panel`计划支持直接兼容`sun-panel`的应用数据导入`sun-panel`的信息！

- 可以**HTTPS**：不管你是否为你的域名配置了HTTPS，为了使用`Next Panel`的完整功能都必须启用`HTTPS`，所以`Next Panel`集成了`Let's Encrypt`，自动处理证书问题。
