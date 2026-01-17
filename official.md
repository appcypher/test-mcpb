# MCP Official Servers List

> Generated from https://github.com/modelcontextprotocol/servers/blob/main/README.md

## Summary

- **Reference Servers**: 7 (maintained by MCP steering group)
- **Official Integrations**: 315+ (from various vendors)

---

## Reference Servers (7)

These are maintained by the MCP steering group in the main repository.

| Server | Description | Transport | Link |
|--------|-------------|-----------|------|
| **Everything** | Reference/test server with prompts, resources, and tools | stdio, SSE, streamable HTTP | [src/everything](https://github.com/modelcontextprotocol/servers/tree/main/src/everything) |
| **Fetch** | Web content fetching and conversion for efficient LLM usage | stdio | [src/fetch](https://github.com/modelcontextprotocol/servers/tree/main/src/fetch) |
| **Filesystem** | Secure file operations with configurable access controls | stdio | [src/filesystem](https://github.com/modelcontextprotocol/servers/tree/main/src/filesystem) |
| **Git** | Tools to read, search, and manipulate Git repositories | stdio | [src/git](https://github.com/modelcontextprotocol/servers/tree/main/src/git) |
| **Memory** | Knowledge graph-based persistent memory system | stdio | [src/memory](https://github.com/modelcontextprotocol/servers/tree/main/src/memory) |
| **Sequential Thinking** | Dynamic and reflective problem-solving through thought sequences | stdio | [src/sequentialthinking](https://github.com/modelcontextprotocol/servers/tree/main/src/sequentialthinking) |
| **Time** | Time and timezone conversion capabilities | stdio | [src/time](https://github.com/modelcontextprotocol/servers/tree/main/src/time) |

---

## Official Integrations (315+)

These are third-party servers listed in the README's "Official Integrations" section. Transport types are **not specified in the README** - each would need to be checked individually.

### Category: Cloud Providers & Infrastructure

| Name | Description | Link |
|------|-------------|------|
| AWS | Specialized MCP servers bringing AWS best practices to development workflows | https://github.com/awslabs/mcp |
| Azure | Access Azure services including Storage, Cosmos DB, and Azure CLI | https://github.com/microsoft/mcp/tree/main/servers/Azure.Mcp.Server |
| Google Cloud Run | Deploy code to Google Cloud Run | https://github.com/GoogleCloudPlatform/cloud-run-mcp |
| Cloudflare | Deploy, configure, and manage Cloudflare resources (Workers/KV/R2/D1) | https://github.com/cloudflare/mcp-server-cloudflare |
| Alibaba Cloud AnalyticDB for MySQL | Connect to AnalyticDB clusters for metadata retrieval and data analysis | https://github.com/aliyun/alibabacloud-adb-mysql-mcp-server |
| Alibaba Cloud AnalyticDB for PostgreSQL | Connect to AnalyticDB PostgreSQL instances for querying and analysis | https://github.com/aliyun/alibabacloud-adbpg-mcp-server |
| Alibaba Cloud DataWorks | Interact with DataWorks Open API for cloud resources operations | https://github.com/aliyun/alibabacloud-dataworks-mcp-server |
| Alibaba Cloud OpenSearch | Equip AI agents with tools to interact with OpenSearch services | https://github.com/aliyun/alibabacloud-opensearch-mcp-server |
| Alibaba Cloud OPS | Manage Alibaba Cloud resource lifecycles via CloudOps Orchestration Service | https://github.com/aliyun/alibaba-cloud-ops-mcp-server |
| Alibaba Cloud RDS | Programmatic management of RDS resources through OpenAPI | https://github.com/aliyun/alibabacloud-rds-openapi-mcp-server |
| Gcore | Interact with Gcore platform services including CDN and cloud resources | https://github.com/G-Core/gcore-mcp-server |
| Heroku | Interact with Heroku Platform for app, add-on, and database management | https://github.com/heroku/heroku-mcp-server |
| Netlify | Create, build, deploy, and manage websites with Netlify | https://docs.netlify.com/welcome/build-with-ai/netlify-mcp-server/ |
| Defang | Deploy projects to the cloud seamlessly with Defang platform | https://github.com/DefangLabs/defang/blob/main/src/pkg/mcp/README.md |
| Daytona | Fast and secure execution of AI-generated code with Daytona sandboxes | https://github.com/daytonaio/daytona/tree/main/apps/cli/mcp |
| E2B | Run code in secure sandboxes hosted by E2B | https://github.com/e2b-dev/mcp-server |
| ForeverVM | Run Python in a code sandbox | https://github.com/jamsocket/forevervm/tree/main/javascript/mcp-server |
| HOPX | Execute Python, JavaScript, Bash, Go in isolated cloud containers | https://github.com/hopx-ai/mcp |
| Hyperbolic | Interact with Hyperbolic's GPU cloud for renting and managing GPUs | https://github.com/HyperbolicLabs/hyperbolic-mcp |
| NanoVMs | Easily build and deploy unikernels to any cloud | https://github.com/nanovms/ops-mcp |

### Category: Databases

| Name | Description | Link |
|------|-------------|------|
| MongoDB | Support for MongoDB Community Server and MongoDB Atlas | https://github.com/mongodb-js/mongodb-mcp-server |
| Neo4j | Neo4j graph database server for schema and cypher operations | https://github.com/neo4j-contrib/mcp-neo4j/ |
| Neo4j GDS | Neo4j graph data science with comprehensive graph algorithms | https://github.com/neo4j-contrib/gds-agent |
| ClickHouse | Query your ClickHouse database server | https://github.com/ClickHouse/mcp-clickhouse |
| Elasticsearch | Query your data in Elasticsearch | https://github.com/elastic/mcp-server-elasticsearch |
| Milvus | Search, query, and interact with data in Milvus vector database | https://github.com/zilliztech/mcp-server-milvus |
| Chroma | Embeddings, vector search, document storage via open-source AI database | https://github.com/chroma-core/chroma-mcp |
| FalkorDB | FalkorDB graph database server for schema exploration and cypher queries | https://github.com/FalkorDB/FalkorDB-MCPServer |
| MariaDB | Standard interface for managing and querying MariaDB databases | https://github.com/mariadb/mcp |
| Neon | Interact with Neon serverless Postgres platform | https://github.com/neondatabase/mcp-server-neon |
| PlanetScale | MySQL-compatible serverless database | https://github.com/planetscale/mcp-server |
| Couchbase | Interact with data stored in Couchbase clusters | https://github.com/Couchbase-Ecosystem/mcp-server-couchbase |
| Astra DB | Comprehensive tools for managing collections and documents in DataStax Astra DB | https://github.com/datastax/astra-db-mcp |
| Apache Doris | MCP server for Apache Doris MPP-based real-time data warehouse | https://github.com/apache/doris-mcp-server |
| Apache IoTDB | MCP server for Apache IoTDB database and tools | https://github.com/apache/iotdb-mcp-server |
| Apache Pinot | Run real-time analytics queries on Apache Pinot OLAP database | https://github.com/startreedata/mcp-pinot |
| GreptimeDB | Secure and structured way to explore and analyze GreptimeDB data | https://github.com/GreptimeTeam/greptimedb-mcp-server |
| Kuzu | Enable LLMs to inspect database schemas and execute queries | https://github.com/kuzudb/kuzu-mcp-server |
| KWDB | Reading, writing, querying data, and DDL operations with KWDB | https://github.com/KWDB/kwdb-mcp-server |
| OceanBase | MCP server for OceanBase database and tools | https://github.com/oceanbase/mcp-oceanbase |
| Memgraph | Query your data in Memgraph graph database | https://github.com/memgraph/ai-toolkit/tree/main/integrations/mcp-memgraph |
| Dolt | Official MCP server for version-controlled Dolt databases | https://github.com/dolthub/dolt-mcp |
| Fireproof | Immutable ledger database with live synchronization | https://github.com/fireproof-storage/mcp-database-server |
| Hologres | Connect to Hologres instance, get metadata, query and analyze data | https://github.com/aliyun/alibabacloud-hologres-mcp-server |
| KurrentDB | Explore data and prototype projections faster on KurrentDB | https://github.com/kurrent-io/mcp-server |
| MCP Toolbox for Databases | Specializing in easy tools for AlloyDB, BigQuery, and more | https://github.com/googleapis/genai-toolbox |
| Meilisearch | Interact and query with Meilisearch full-text and semantic search API | https://github.com/meilisearch/meilisearch-mcp |
| MotherDuck | Query and analyze data with MotherDuck and DuckDB | https://github.com/motherduckdb/mcp-server-motherduck |
| Nile | Manage and query databases with Nile Postgres for B2B apps | https://github.com/niledatabase/nile-mcp-server |

### Category: Developer Tools & Version Control

| Name | Description | Link |
|------|-------------|------|
| GitHub | GitHub's official MCP server | https://github.com/github/github-mcp-server |
| GitLab | GitLab's official MCP server for secure project data access | https://docs.gitlab.com/user/gitlab_duo/model_context_protocol/mcp_server/ |
| Gitea | Interact with Gitea instances with MCP | https://gitea.com/gitea/gitea-mcp |
| Gitee | Gitee API integration for repository and issue management | https://github.com/oschina/mcp-gitee |
| AtomGit | Official server for repository management, PRs, issues, branches, labels | https://atomgit.com/atomgit-open-source-ecosystem/atomgit-mcp-server |
| GitKraken | CLI for GitKraken APIs wrapping GitKraken, Jira, GitHub, GitLab | https://github.com/gitkraken/gk-cli?tab=readme-ov-file#mcp-server |
| GitGuardian | Scan projects using GitGuardian's 500+ secret detectors | https://github.com/GitGuardian/gg-mcp |
| JetBrains | Work on code with JetBrains IDEs (IntelliJ IDEA, PhpStorm, etc.) | https://www.jetbrains.com/help/idea/mcp-server.html |
| Jenkins | Official Jenkins MCP server plugin for build management integration | https://plugins.jenkins.io/mcp-server/ |
| CircleCI | Enable AI agents to fix build failures from CircleCI | https://github.com/CircleCI-Public/mcp-server-circleci |
| Buildkite | Exposing Buildkite data (pipelines, builds, jobs, tests) to AI tooling | https://github.com/buildkite/buildkite-mcp-server |
| Azure DevOps | Interact with Azure DevOps repositories, work items, builds, releases | https://github.com/microsoft/azure-devops-mcp |
| Bitrise | Chat with your builds, CI, and related Bitrise platform features | https://github.com/bitrise-io/bitrise-mcp |
| JFrog | MCP server for JFrog Platform API with repository and build management | https://github.com/jfrog/mcp-jfrog |
| Nx | Makes Nx's understanding of codebase accessible to LLMs | https://github.com/nrwl/nx-console/blob/master/apps/nx-mcp |
| Chrome DevTools | Enable AI coding assistants to debug web pages directly in Chrome | https://github.com/ChromeDevTools/chrome-devtools-mcp |
| Codacy | Interact with Codacy API for code quality issues and coverage insights | https://github.com/codacy/codacy-mcp-server/ |
| CodeLogic | Software Intelligence platform graphing code and data architecture dependencies | https://github.com/CodeLogicIncEngineering/codelogic-mcp-server |

### Category: Project Management & Collaboration

| Name | Description | Link |
|------|-------------|------|
| Atlassian | Securely interact with Jira work items and Confluence pages | https://www.atlassian.com/platform/remote-mcp-server |
| Linear | Search, create, and update Linear issues, projects, and comments | https://linear.app/docs/mcp |
| Notion | Official MCP server for the Notion API | https://github.com/makenotion/notion-mcp-server#readme |
| Monday.com | Interact with Monday.com boards, items, accounts, and work forms | https://github.com/mondaycom/mcp |
| Dart | Interact with task, doc, and project data in Dart project management tool | https://github.com/its-dart/dart-mcp-server |
| Fibery | Perform queries and entity operations in your Fibery workspace | https://github.com/Fibery-inc/fibery-mcp-server |
| Plane | Project management and issue tracking | https://github.com/makeplane/plane/tree/develop/mcp |
| Atono | Create and update stories, bugs, assignments for modern product teams | https://github.com/Atono/atono-mcp-server |

### Category: Communication & Messaging

| Name | Description | Link |
|------|-------------|------|
| Microsoft Teams | Official Teams AI Library with MCP support | https://devblogs.microsoft.com/microsoft365dev/announcing-the-updated-teams-ai-library-and-mcp-support/ |
| LINE | Integrates LINE Messaging API to connect AI agent to LINE Official Account | https://github.com/line/line-bot-mcp-server |
| Infobip | Integrate global cloud communication platform for SMS, RCS, WhatsApp | https://github.com/infobip/mcp |
| Courier | Build, update, and send multi-channel notifications (email, SMS, push, Slack, Teams) | https://www.courier.com/docs/tools/mcp |
| ClickSend | Official ClickSend MCP server for communication capabilities | https://github.com/ClickSend/clicksend-mcp-server/ |
| Knock MCP Server | Send product and customer messaging across multiple channels | https://github.com/knocklabs/agent-toolkit#model-context-protocol-mcp |

### Category: Email

| Name | Description | Link |
|------|-------------|------|
| Mailgun | Interact with Mailgun API for email services | https://github.com/mailgun/mailgun-mcp-server |
| Mailjet | Interact with contact, campaign, and statistics from Mailjet | https://github.com/mailgun/mailjet-mcp-server |
| Elastic Email | Full-scale email capabilities for AI agents | https://github.com/ElasticEmail/elasticemail-mcp-server |
| Postmark | Transactional email service | https://github.com/postmark/mcp-server |
| Inbox Zero | AI personal assistant for email management | https://github.com/elie222/inbox-zero/tree/main/apps/mcp-server |

### Category: CRM & Sales

| Name | Description | Link |
|------|-------------|------|
| HubSpot | Connect, manage, and interact with HubSpot CRM data | https://developer.hubspot.com/mcp |
| Pipedrive | Sales pipeline and CRM management | https://github.com/pipedrive/mcp-server |
| Hunter | Interact with Hunter API to get B2B data using natural language | https://github.com/hunter-io/hunter-mcp |
| Explorium | B2B data and infrastructure for AI SDR and GTM agents | https://github.com/explorium-ai/mcp-explorium |

### Category: Analytics & Observability

| Name | Description | Link |
|------|-------------|------|
| Grafana | Search dashboards, investigate incidents, query datasources in Grafana | https://github.com/grafana/mcp-grafana |
| Mixpanel | Query and analyze product analytics data through natural language | https://docs.mixpanel.com/docs/features/mcp |
| Amplitude | Seamless integration with product data for searching and analyzing charts, dashboards, and metrics | https://amplitude.com/docs/analytics/amplitude-mcp |
| PostHog | Product analytics and feature flags | https://github.com/PostHog/posthog-mcp |
| Axiom | Query and analyze Axiom logs, traces, and event data in natural language | https://github.com/axiomhq/mcp-server-axiom |
| Honeycomb | Query, analyze data, alerts, dashboards; cross-reference production behavior | https://github.com/honeycombio/honeycomb-mcp |
| Dynatrace | Manage and interact with Dynatrace Platform for real-time observability | https://github.com/dynatrace-oss/dynatrace-mcp |
| Last9 | Bring real-time production context—logs, metrics, and traces | https://github.com/last9/last9-mcp-server |
| Logfire | Access OpenTelemetry traces and metrics through Logfire | https://github.com/pydantic/logfire-mcp |
| Netdata | Discovery, exploration, reporting of observability data | https://github.com/netdata/netdata/blob/master/src/web/mcp/README.md |
| Microsoft Clarity | Official MCP server to get behavioral analytics from Clarity | https://github.com/microsoft/clarity-mcp-server |
| AgentOps | Observability and tracing for debugging AI agents using AgentOps API | https://github.com/AgentOps-AI/agentops-mcp |

### Category: Security

| Name | Description | Link |
|------|-------------|------|
| CrowdStrike Falcon | Connect AI agents with CrowdStrike Falcon for security analysis | https://github.com/CrowdStrike/falcon-mcp |
| Contrast Security | Brings vulnerability and SCA data into your coding agent | https://github.com/Contrast-Security-OSS/mcp-contrast |
| Cycode | Boost security in development lifecycle via SAST, SCA, Secrets scanning | https://github.com/cycodehq/cycode-cli#mcp-command-experiment |
| Endor Labs | Find and fix security risks via SAST, SCA, Secrets scanning | https://docs.endorlabs.com/deployment/ide/mcp/ |
| BoostSecurity | Guardrails coding agents against vulnerable dependencies and malware | https://github.com/boost-community/boost-mcp |
| Burp Suite | MCP server extension allowing AI clients to connect to Burp Suite | https://github.com/PortSwigger/mcp-server |
| Fluid Attacks | Interact with Fluid Attacks API for vulnerability management | https://github.com/fluidattacks/mcp |
| Mobb | Identify and remediate vulnerabilities in code | https://github.com/mobb-dev/bugsy?tab=readme-ov-file#model-context-protocol-mcp-server |
| Gremlin | Official Gremlin MCP server for reliability analysis and testing | https://github.com/gremlin/mcp |

### Category: AI & ML Platforms

| Name | Description | Link |
|------|-------------|------|
| Hugging Face | Connect to Hugging Face Hub APIs: search, explore datasets, models | https://huggingface.co/settings/mcp |
| Arize Phoenix | Inspect traces, manage prompts, curate datasets, and run experiments | https://github.com/Arize-ai/phoenix/tree/main/js/packages/phoenix-mcp |
| Comet Opik | Query and analyze logs, traces, prompts and telemetry data | https://github.com/comet-ml/opik-mcp |
| Langfuse Prompt Management | Open-source tool for collaborative editing and versioning | https://github.com/langfuse/mcp-server-langfuse |
| Atla | Enable AI agents to interact with Atla API for state-of-the-art LLM evaluation | https://github.com/atla-ai/atla-mcp-server |
| Kiln | Free open-source platform for building production-ready AI systems | https://github.com/Kiln-AI/Kiln |

### Category: Web Scraping & Browser Automation

| Name | Description | Link |
|------|-------------|------|
| Firecrawl | Extract web data with Firecrawl | https://github.com/firecrawl/firecrawl-mcp-server |
| Apify | Use 6,000+ pre-built cloud tools to extract data from websites and platforms | https://github.com/apify/apify-mcp-server |
| BrightData | Discover, extract, and interact with public internet data | https://github.com/luminati-io/brightdata-mcp |
| Browserbase | Automate browser interactions in the cloud for navigation and data extraction | https://github.com/browserbase/mcp-server-browserbase |
| Hyperbrowser | Enable effortless, scalable browser automation platform | https://github.com/hyperbrowserai/mcp |
| AgentQL | Enable AI agents to extract structured data from unstructured web content | https://github.com/tinyfish-io/agentql-mcp |
| Dumpling AI | Access data, web scraping, and document conversion APIs | https://github.com/Dumpling-AI/mcp-server-dumplingai |
| GoLogin MCP server | Manage GoLogin browser profiles and automation | https://github.com/gologinapp/gologin-mcp |
| Kernel | Access Kernel's cloud-based browsers via MCP | https://github.com/onkernel/kernel-mcp-server |
| PlayWright | Browser automation testing | https://github.com/microsoft/playwright/tree/main/mcp |
| BrowserStack | Access BrowserStack's test platform for debugging and accessibility testing | https://github.com/browserstack/mcp-server |
| LambdaTest | LambdaTest MCP servers for accessibility, SmartUI, automation testing | https://www.lambdatest.com/mcp |

### Category: Search

| Name | Description | Link |
|------|-------------|------|
| Exa | Search engine made for AIs by Exa | https://github.com/exa-labs/exa-mcp-server |
| Kagi Search | Search the web using Kagi's search API | https://github.com/kagisearch/kagimcp |
| Algolia | Provision, configure, and query Algolia search indices via AI agents | https://github.com/algolia/mcp |
| Linkup (JS) | Web search MCP server through Linkup's advanced search API | https://github.com/LinkupPlatform/js-mcp-server |
| Linkup (Python) | Web search MCP server through Linkup's advanced search API | https://github.com/LinkupPlatform/python-mcp-server |
| Cloudsway SmartSearch | Web search MCP server powered by Cloudsway with structured JSON results | https://github.com/Cloudsway-AI/smartsearch |

### Category: Payments & Finance

| Name | Description | Link |
|------|-------------|------|
| Cashfree | Cashfree Payments official MCP server | https://github.com/cashfree/cashfree-mcp |
| Flutterwave | Interact with Flutterwave payment solutions for transactions | https://github.com/bajoski34/mcp-flutterwave/tree/main |
| AlipayPlus | Connect AI agents to AlipayPlus Checkout Payment services | https://github.com/alipay/global-alipayplus-mcp |
| Antom | Connect AI agents to Antom Checkout Payment services | https://github.com/alipay/global-antom-mcp |
| Bitnovo Pay | Cryptocurrency payment integration for Bitcoin, Ethereum, and other cryptocurrencies | https://github.com/bitnovo/mcp-bitnovo-pay |
| Alpaca | Trade stocks and options, analyze market data via Alpaca's Trading API | https://github.com/alpacahq/alpaca-mcp-server |
| AlphaVantage | Connect to 100+ APIs for financial market data | https://mcp.alphavantage.co/ |
| Financial Datasets | Stock market API made for AI agents | https://github.com/financial-datasets/mcp-server |
| Plaid | Financial institution connections and account data access | https://github.com/plaid/mcp-server |
| Chargebee | MCP server connecting AI agents to Chargebee subscription management platform | https://github.com/chargebee/agentkit/tree/main/modelcontextprotocol |
| Commerce Layer | Interact with Commerce Layer Metrics API | https://github.com/commercelayer/mcp-server-metrics |
| Norman Finance | MCP server for managing accounting and taxes | https://github.com/norman-finance/norman-mcp-server |

### Category: Crypto & Blockchain

| Name | Description | Link |
|------|-------------|------|
| CoinGecko | Official CoinGecko API server for crypto price and market data | https://github.com/coingecko/coingecko-typescript/tree/main/packages/mcp-server |
| CoinStats | MCP server for CoinStats API providing market data and portfolio tracking | https://github.com/CoinStatsHQ/coinstats-mcp |
| Coinex | Official Coinex API MCP server for cryptocurrency exchange operations | https://github.com/coinexcom/coinex_mcp_server |
| Armor Crypto MCP | Interface with blockchains, staking, DeFi, swaps, bridges, wallet management | https://github.com/armorwallet/armor-crypto-mcp |
| Alby Bitcoin Payments | Connect bitcoin lightning wallets for instant global payments | https://github.com/getAlby/mcp |
| BICScan | Risk score and asset holdings lookup for EVM blockchain addresses | https://github.com/ahnlabio/bicscan-mcp |
| Bankless Onchain | Query onchain data like ERC20 tokens and transaction history | https://github.com/bankless/onchain-mcp |
| Behavioural Prediction | AI-powered wallet behavior analysis and fraud detection by ChainAware.ai | https://github.com/ChainAware/behavioral-prediction-mcp |
| DexPaprika | Access real-time DEX data, liquidity pools, and trading analytics | https://github.com/coinpaprika/dexpaprika-mcp |
| Ember AI | Unified MCP server enabling AI agents to execute cross-chain DeFi strategies | https://docs.emberai.xyz/ |
| Fewsats | Enable AI agents to purchase items securely via Fewsats | https://github.com/Fewsats/fewsats-mcp |
| Hive Intelligence | Ultimate cryptocurrency MCP for crypto, DeFi, and Web3 analytics | https://github.com/hive-intel/hive-crypto-mcp |
| Nodit | Official Nodit MCP server for multi-chain RPC nodes and data APIs | https://github.com/noditlabs/nodit-mcp-server |

### Category: Authentication & Identity

| Name | Description | Link |
|------|-------------|------|
| Auth0 | MCP server for Auth0 tenant interactions supporting actions, applications, forms, logs | https://github.com/auth0/auth0-mcp-server |
| Okta | Manage identity and access through Okta's platform | https://github.com/okta/okta-mcp-server |
| Asgardeo | MCP server to interact with Asgardeo organization through LLM tools | https://github.com/asgardeo/asgardeo-mcp-server |
| cheqd | Enable AI agents to be trusted and verified through cheqd's trust registries | https://github.com/cheqd/mcp-toolkit |
| Authenticator App 2FA | Secure MCP server enabling AI agents to interact with Authenticator App | https://github.com/firstorderai/authenticator_mcp |

### Category: Media & Content

| Name | Description | Link |
|------|-------------|------|
| Cloudinary | Media upload, transformation, AI analysis, management, and optimization | https://github.com/cloudinary/mcp-servers |
| Gyazo | Search, fetch, upload, and interact with Gyazo images with OCR data | https://github.com/nota/gyazo-mcp-server |
| AllVoiceLab | AI voice toolkit with TTS, voice cloning, and video translation | https://www.allvoicelab.com/mcp |
| Cartesia | Connect to Cartesia voice platform for text-to-speech and voice cloning | https://github.com/cartesia-ai/cartesia-mcp |
| DeepL | Translate or rewrite text with DeepL's AI models | https://github.com/DeepLcom/deepl-mcp-server |
| Lara Translate | MCP server for Lara Translate API with language detection support | https://github.com/translated/lara-mcp |
| Lingo.dev | Make AI agents speak every language using Lingo.dev Localization Engine | https://github.com/lingodotdev/lingo.dev/blob/main/mcp.md |

### Category: Documentation & Knowledge

| Name | Description | Link |
|------|-------------|------|
| Microsoft Learn Docs | Structured access to Microsoft's official documentation | https://github.com/microsoftdocs/mcp |
| DevExpress | Documentation MCP server with instant access to 300,000+ help topics | https://docs.devexpress.com/GeneralInformation/405551/help-resources/dev-express-documentation-mcp-server-configure-an-ai-powered-assistant |
| Inkeep | RAG search over your content powered by Inkeep | https://github.com/inkeep/mcp-server-python |
| Needle | Production-ready RAG to search and retrieve data from documents | https://github.com/needle-ai/needle-mcp |
| Graphlit | Ingest content from Slack, Gmail, podcasts into searchable Graphlit project | https://github.com/graphlit/graphlit-mcp-server |
| Archbee | Write and publish documentation that becomes trusted source for instant answers | https://www.npmjs.com/package/@archbee/mcp |
| Glean | Enterprise search and chat using Glean's API | https://github.com/gleanwork/mcp-server |

### Category: E-commerce

| Name | Description | Link |
|------|-------------|------|
| Mercado Libre | Mercado Libre's official MCP server | https://mcp.mercadolibre.com/ |
| Mercado Pago | Mercado Pago's official MCP server | https://mcp.mercadopago.com/ |
| MCP for WooCommerce | Connect WooCommerce store to AI assistants with read-only access | https://github.com/iOSDevSK/mcp-for-woocommerce |
| Shopify Admin MCP | Shopify Admin task control for agents | (from registry) |

### Category: Smart Home & IoT

| Name | Description | Link |
|------|-------------|------|
| Aqara MCP Server | Control Aqara smart home devices and execute scenes via natural language | https://github.com/aqara/aqara-mcp-server/ |
| ESP RainMaker | Official Espressif MCP server to control and manage devices | https://github.com/espressif/esp-rainmaker-mcp |
| Philips Hue | Smart lighting control | https://github.com/philips-hue/mcp-server |

### Category: Maps & Location

| Name | Description | Link |
|------|-------------|------|
| Baidu Map | Tools for AI agents to interact with Baidu Maps APIs for location-based services | https://github.com/baidu-maps/mcp |
| Mapbox | Unlock geospatial intelligence through Mapbox APIs | https://github.com/mapbox/mcp-server |
| Google Maps Platform Code Assist | Ground agents on fresh, official documentation | https://github.com/googlemaps/platform-ai/tree/main/packages/code-assist |
| IP2Location.io | Retrieve geolocation information for IP addresses | https://github.com/ip2location/mcp-ip2location-io |
| IPLocate | Look up IP address geolocation and detect proxies, VPNs | https://github.com/iplocate/mcp-server-iplocate |
| Globalping | Access network of thousands of probes for network commands | https://github.com/jsdelivr/globalping-mcp-server |

### Category: Data Integration & ETL

| Name | Description | Link |
|------|-------------|------|
| Databricks | Connect to data, AI tools, agents, and rest of Databricks platform | https://docs.databricks.com/aws/en/generative-ai/mcp/ |
| DataHub | Search data assets, traverse lineage, write SQL queries using DataHub metadata | https://github.com/acryldata/mcp-server-datahub |
| Confluent | Interact with Confluent Kafka and Confluent Cloud REST APIs | https://github.com/confluentinc/mcp-confluent |
| Keboola | Build robust data workflows, integrations, and analytics | https://github.com/keboola/keboola-mcp-server |
| Nango | Integrate AI agent with 500+ APIs with auth and observability | https://nango.dev/docs/guides/use-cases/ai-tool-calling |
| Composio | Connect to 100+ tools. Zero setup. Auth built-in. | https://docs.composio.dev/docs/mcp-overview#-getting-started |
| Integration App | Interact with any SaaS applications on behalf of customers | https://github.com/integration-app/mcp-server |
| Knit MCP Server | Production-ready servers connecting with 10000+ tools | https://developers.getknit.dev/docs/knit-mcp-server-getting-started |
| Make | Turn Make scenarios into callable tools for AI assistants | https://github.com/integromat/make-mcp-server |
| IBM wxflows | Tool platform by IBM to build, test, and deploy tools | https://github.com/IBM/wxflows/tree/main/examples/mcp/javascript |

### Category: Scheduling & Calendar

| Name | Description | Link |
|------|-------------|------|
| Cal.com | Connect to Cal.com API to schedule and manage bookings and appointments | https://www.npmjs.com/package/@calcom/cal-mcp?activeTab=readme |
| Kalendis | Generate TypeScript clients and API route handlers for scheduling API | https://github.com/kalendis-dev/kalendis-mcp |
| Grain | Access Grain meeting notes, transcripts, and generate reports | https://grain.com/release-note/06-18-2025 |

### Category: Feature Flags & Config

| Name | Description | Link |
|------|-------------|------|
| LaunchDarkly | Continuous delivery platform providing feature flags as a service | https://github.com/launchdarkly/mcp-server |
| ConfigCat | Manage feature flags, configs, environments with ConfigCat service | https://github.com/configcat/mcp-server |
| DevCycle | Create and monitor feature flags using natural language in AI assistants | https://docs.devcycle.com/cli-mcp/mcp-getting-started |

### Category: Design & UI

| Name | Description | Link |
|------|-------------|------|
| 21st.dev Magic | Create crafted UI components inspired by the best 21st.dev design engineers | https://github.com/21st-dev/magic-mcp |
| Canva | Provide AI-powered development assistance for Canva apps and integrations | https://www.canva.dev/docs/apps/mcp-server/ |
| Datawrapper | Create Datawrapper charts using AI assistants | https://github.com/palewire/datawrapper-mcp |
| gNucleus Text-To-CAD | Generate CAD parts and assemblies from text | https://github.com/gNucleus/text-to-cad-mcp |

### Category: Storage & Files

| Name | Description | Link |
|------|-------------|------|
| Box | Interact with Box intelligent content management through Box AI | https://github.com/box-community/mcp-server-box |
| CTERA Edge Filer | Intelligent edge caching and multiprotocol file access | https://github.com/ctera/mcp-ctera-edge |
| CTERA Portal | Multi-tenant, multi-cloud platform with global namespace | https://github.com/ctera/mcp-ctera-core |
| Kiteworks | Official MCP server to interact with Kiteworks PDN platform | https://github.com/kiteworks/mcp |

### Category: Workflow & Automation

| Name | Description | Link |
|------|-------------|------|
| Conductor | Interact with Conductor (OSS and Orkes) REST APIs | https://github.com/conductor-oss/conductor-mcp |
| Pipefy | Workflow automation and process management | https://github.com/pipefy/mcp-server |
| Hiveflow | Create, manage, and execute agentic AI workflows | https://github.com/hiveflowai/hiveflow-mcp-server |

### Category: Testing & QA

| Name | Description | Link |
|------|-------------|------|
| Appium MCP Server | MCP server for mobile development and automation across iOS, Android, simulators, emulators | https://github.com/appium/appium-mcp.git |
| AltTester | Connect and test Unity or Unreal games with AltTester capabilities | https://alttester.com/docs/desktop/latest/pages/ai-extension.html |
| Debugg.AI | Zero-Config, Fully AI-Managed End-to-End Testing for code generation platforms | https://github.com/debugg-ai/debugg-ai-mcp |
| Lippia | MCP server to accelerate test automation using Lippia Framework | https://github.com/Lippia-io/Lippia-MCP-Server/blob/main/getting-started.md |
| Inflectra Spira | Connect to SpiraTest, SpiraTeam, or SpiraPlan ALM platform | https://github.com/Inflectra/mcp-server-spira |
| Multiplayer | Analyze full stack session recordings and fix bugs | https://www.multiplayer.app/docs/ai/mcp-server |

### Category: API Development

| Name | Description | Link |
|------|-------------|------|
| APIMatic MCP | Validate OpenAPI specifications using APIMatic's validation service | https://github.com/apimatic/apimatic-validator-mcp |
| Apollo MCP Server | Connect GraphQL APIs to AI agents | https://github.com/apollographql/apollo-mcp-server/ |
| Grafbase | Turn GraphQL API into efficient MCP server with schema intelligence | https://github.com/grafbase/grafbase/tree/main/crates/mcp |
| AgentRPC | Connect to any function, any language, across network boundaries | https://github.com/agentrpc/agentrpc |

### Category: Other Official Integrations (A-N continued)

| Name | Description | Link |
|------|-------------|------|
| 2slides | Tools to transform content into presentations or generate slides aligned with user intent | https://github.com/2slides/2slides-mcp |
| ActionKit by Paragon | Connect to 130+ SaaS integrations (e.g. Slack, Salesforce, Gmail) via Paragon's ActionKit API | https://github.com/useparagon/paragon-mcp |
| Adfin | All-in-one payment platform providing invoicing and accounting reconciliation services | https://github.com/Adfin-Engineering/mcp-server-adfin |
| Agentset | RAG capabilities for knowledge bases connected to Agentset platform | https://github.com/agentset-ai/mcp-server |
| Airwallex Developer | Empowers coding agents with tools for Airwallex API integration | https://www.npmjs.com/package/@airwallex/developer-mcp |
| Aiven | Navigate Aiven projects and interact with PostgreSQL, Kafka, ClickHouse, OpenSearch services | https://github.com/Aiven-Open/mcp-aiven |
| Alation | Unlock enterprise Data Catalog capabilities through Alation MCP server | https://github.com/Alation/alation-ai-agent-sdk |
| Alkemi | Query Snowflake, BigQuery, DataBricks via Alkemi.ai platform | https://github.com/alkemi-ai/alkemi-mcp |
| Anytype | Interact with Anytype platform to organize objects and lists through natural language | https://github.com/anyproto/anytype-mcp |
| Atlan | Interact with Atlan services through multiple tools | https://github.com/atlanhq/agent-toolkit/tree/main/modelcontextprotocol |
| Audiense Insights | Marketing insights and audience analysis from Audiense reports | https://github.com/AudienseCo/mcp-audiense-insights |
| Backdocket | Search, retrieve, and update Backdocket claims, matters, contacts, tasks | https://ai.backdocket.com |
| Baserow | Query data from Baserow self-hosted or SaaS databases | https://gitlab.com/baserow/baserow/-/tree/develop/backend/src/baserow/api/mcp |
| Bauplan | Manage the Bauplan lakehouse: query tables, create branches, run pipelines | https://github.com/BauplanLabs/bauplan-mcp-server |
| Boikot | Learn about ethical and unethical actions of major companies | https://github.com/boikot-xyz/boikot |
| BoldSign | Search, request, and manage e-signature contracts via BoldSign platform | https://github.com/boldsign/boldsign-mcp |
| Boost.space | Integrating with Boost.space for centralized automated business data | https://github.com/boostspace/boostspace-mcp-server |
| Buildable | Official MCP server for Buildable AI-powered development platform | https://github.com/chunkydotdev/bldbl-mcp |
| BuiltWith | Identify the technology stack behind any website | https://github.com/builtwith/mcp |
| Campertunity | Search campgrounds worldwide, check availability, and get booking links | https://github.com/campertunity/mcp-server |
| Carbon Voice | Connect AI agents to Carbon Voice for managing conversations and voice interactions | https://github.com/PhononX/cv-mcp-server |
| CB Insights | Connect to ChatCBI using CB Insights MCP server | https://github.com/cbinsights/cbi-mcp-server |
| Chiki StudIO | Create configurable MCP servers via configuration without coding | https://chiki.studio/galimybes/mcp/ |
| Chronulus AI | Predict anything with Chronulus AI forecasting and prediction agents | https://github.com/ChronulusAI/chronulus-mcp |
| Claude Context | Bring your codebase as context to Claude Code | https://github.com/zilliztech/claude-context |
| Cleanup Crew | Real-time human support service for non-technical founders using AI coding tools | https://cleanupcrew.ai/install |
| Clix MCP Server | Real-time Clix documentation and SDK code examples for integrations | https://github.com/clix-so/clix-mcp-server |
| CloudBase | One-stop backend services for WeChat Mini-Programs and full-stack apps | https://github.com/TencentCloudBase/CloudBase-AI-ToolKit |
| CloudBees CI | Enable AI access to CloudBees CI cluster for build management | https://docs.cloudbees.com/docs/cloudbees-ci-mcp-router/latest/ |
| CloudBees Unify | Enable AI access to CloudBees Unify environment | https://docs.cloudbees.com/docs/cloudbees-unify-mcp-server/latest/install/mcp-server |
| Cloudbet | Structured sports and esports data including fixtures, odds, and markets | https://github.com/cloudbet/sports-mcp-server |
| Cloudera Iceberg | Enable AI on the Open Data Lakehouse | https://github.com/cloudera/iceberg-mcp-server |
| Construe | FastMCP server for intelligent Obsidian vault context management | https://github.com/mattjoyce/mcp-construe |
| Context Templates | Open-source collection of reusable context templates for development | https://github.com/ginylil/context-templates |
| Convex | Introspect and query apps deployed to Convex | https://stack.convex.dev/convex-mcp-server |
| Cortex | Official MCP server for Cortex platform | https://github.com/cortexapps/cortex-mcp |
| CRIC Wuye AI | Interact with CRIC Wuye AI platform for property management | https://github.com/wuye-ai/mcp-server-wuye-ai |
| Customer.io | Work with Customer.io workspace to create segments and manage audiences | https://docs.customer.io/ai/mcp-server/ |
| DeepQ | Chinese Financial AI toolkit providing financial data and analytical tools | https://github.com/shenqingtech/deepq-financial-toolkit-mcp-server |
| DeployHQ | MCP server for DeployHQ API integration managing deployments | https://github.com/deployhq/deployhq-mcp-server |
| Destinia | Search for hotels and get listing details from Destinia | https://destinia.com/developers |
| Detailer | Instantly generate rich, AI-powered documentation for GitHub repositories | https://detailer.ginylil.com/ |
| DevHub | Manage and utilize website content within DevHub CMS platform | https://github.com/devhub/devhub-cms-mcp |
| DevRev | MCP server to search through DevRev Knowledge Graph | https://github.com/devrev/mcp-server |
| Diffusion | Connect to Diffusion servers to explore and manage topics | https://github.com/diffusiondata/diffusion-mcp-server |
| Dot (GetDot.ai) | Fetch, analyze, visualize data from favorite databases | https://docs.getdot.ai/dot/integrations/mcp |
| Drata | Real-time compliance intelligence MCP server | https://drata.com/mcp |
| Edgee | Deploy and manage Edgee components and projects | https://github.com/edgee-cloud/mcp-server-edgee |
| EduBase | Interact with EduBase e-learning platform for content and assessment management | https://github.com/EduBase/MCP |
| Elasticsearch Memory | Persistent memory with hierarchical categorization and semantic search | https://github.com/fredac100/elasticsearch-memory-mcp |
| eSignatures | Contract and template management for drafting and sending binding contracts | https://github.com/esignaturescom/mcp-server-esignatures |
| fetchSERP | All-in-One SEO and Web Intelligence Toolkit API | https://github.com/fetchSERP/fetchserp-mcp-server-node |
| Firebase | Firebase's experimental MCP server to power AI tools | https://github.com/firebase/firebase-tools/blob/master/src/mcp |
| Firefly | Integrates, discovers, manages, and codifies cloud resources | https://github.com/gofireflyio/firefly-mcp |
| FIXParser | Modern FIX Protocol engine for AI-powered trading agents | https://gitlab.com/logotype/fixparser/-/tree/main/packages/fixparser-plugin-mcp |
| GibsonAI | AI-Powered Cloud databases: build, migrate, and deploy instances | https://github.com/GibsonAI/mcp |
| gotoHuman | Human-in-the-loop platform for approval request integration | https://github.com/gotohuman/gotohuman-mcp-server |
| GROWI | Official MCP server to integrate with GROWI APIs | https://github.com/growilabs/growi-mcp-server |
| Harper | MCP server providing interface to access data in Harper platform | https://github.com/HarperDB/mcp-server |
| HeyOnCall | Page a human by sending alerts to HeyOnCall iOS or Android apps | https://heyoncall.com/blog/mcp-server-for-paging-a-human |
| Hillnote | Search, edit, save, and create markdown documents to workspace | https://github.com/Rajathbail/hillnote-mcp-server |
| Homebrew | Run Homebrew commands locally for package management | https://docs.brew.sh/MCP-Server |
| IBM watsonx.data intelligence | Find, understand, and work with data through governance & catalog | https://github.com/IBM/data-intelligence-mcp-server |
| Improve Digital Publisher MCP | Integrate Improve Digital's inventory management system | https://github.com/azerion/improvedigital-publisher-mcp-server |
| Jellyfish | Give AI agent context about team's software engineering allocations | https://github.com/Jellyfish-AI/jellyfish-mcp |
| Kaltura | Manage Kaltura Event Platform for virtual events | https://github.com/kaltura/mcp-events |
| Kash.click | Give AI access to sales, clients, orders, tax information, and business insights | https://github.com/paracetamol951/caisse-enregistreuse-mcp-server |
| Keywords Everywhere | Access SEO data through official Keywords Everywhere API | https://api.keywordseverywhere.com/docs/#/mcp_integration |
| KeywordsPeopleUse.com | Find questions people ask online | https://github.com/data-skunks/kpu-mcp |
| Kintone | Official local MCP server for Kintone | https://github.com/kintone/mcp-server |
| KirokuForms | AI-powered form platform combining form building with HITL | https://www.kirokuforms.com/ai/mcp |
| Klavis ReportGen | Create professional reports from simple user queries | https://github.com/Klavis-AI/klavis/tree/main/mcp_servers/report_generation |
| Klaviyo | Interact with your Klaviyo marketing data | https://developers.klaviyo.com/en/docs/klaviyo_mcp_server |
| kluster.ai | MCP servers bringing AI services with guardrails like hallucination detection | https://docs.kluster.ai/get-started/mcp/overview/ |
| Kumo | MCP server to interact with KumoRFM foundation model | https://github.com/kumo-ai/kumo-rfm-mcp |
| kweenkl | Send push notifications from AI assistants using natural language | https://github.com/antoinedelorme/kweenkl-mcp |
| Label Studio | Open source data labeling platform | https://github.com/HumanSignal/label-studio-mcp-server |
| Lambda Capture | Macroeconomic forecasts and semantic context from central banks | https://github.com/lambda-capture/mcp-server |
| LinkedIn MCP Runner | Write, edit, and schedule LinkedIn posts from ChatGPT and Claude | https://github.com/ertiqah/linkedin-mcp-runner |
| Litmus.io | Official MCP server for configuring Litmus Edge | https://github.com/litmusautomation/litmus-mcp-server |
| Liveblocks | Ready-made features for AI and human collaboration | https://github.com/liveblocks/liveblocks-mcp-server |
| Magic Meal Kits | Unleash Make's full potential via Magic Meal Kits | https://github.com/pureugong/mmk-mcp |
| MCP Discovery | Lightweight CLI tool for discovering MCP server capabilities | https://github.com/rust-mcp-stack/mcp-discovery |
| Memalot | Finds memory leaks in Python programs | https://github.com/nfergu/memalot?tab=readme-ov-file#mcp-server |
| Metoro | Query and interact with kubernetes environments monitored by Metoro | https://github.com/metoro-io/metoro-mcp-server |
| Microsoft Business Central | Manage Dynamics 365 Business Central customers and invoices | https://github.com/knowall-ai/mcp-business-central |
| Microsoft Dataverse | Chat over business data, discover tables, run queries | https://go.microsoft.com/fwlink/?linkid=2320176 |
| mimilabs | US healthcare data discovery guide for 50+ gov sources | https://www.mimilabs.ai/mcp |
| Momento | Momento cache for improving performance and handling load | https://github.com/momentohq/mcp-momento |
| Moorcheh | Integration with Moorcheh's embedding and AI answer services | https://github.com/moorcheh-ai/moorcheh-mcp |
| Mulesoft | Build, deploy, and manage MuleSoft applications | https://www.npmjs.com/package/@mulesoft/mcp-server |
| Neo4j Agent Memory | Memory management for AI agents using Neo4j knowledge graphs | https://github.com/knowall-ai/mcp-neo4j-agent-memory |
| Nerve | Search and act on all company data across SaaS apps | https://github.com/nerve-hq/nerve-mcp-server |
| NetApp | Query metrics, manage volumes, and search NetApp systems | https://github.com/NetApp/mcp |
| Notifly | Real-time Notifly documentation and SDK code examples | https://github.com/notifly-tech/notifly-mcp-server |
| Nutrient | Create, edit, sign, extract documents using natural language | https://github.com/PSPDFKit/nutrient-dws-mcp-server |

---

## Transport Type Notes

- **stdio**: Standard input/output - most common for local servers
- **SSE (Server-Sent Events)**: HTTP-based streaming for remote servers
- **Streamable HTTP**: HTTP transport with streaming support

Most reference servers use **stdio** transport. The "Everything" reference server uniquely supports all three transport types for testing purposes.

For Official Integrations, transport types vary by implementation and are not documented in the main README. Check individual repositories for specific transport support.

---

## Data Sources

- Main README: https://github.com/modelcontextprotocol/servers/blob/main/README.md
- MCP Registry API: https://registry.modelcontextprotocol.io/v0.1/servers
- Source code analysis for transport determination

---

*Generated: 2026-01-08*
