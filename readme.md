# AzureScript

**AzureScript** is a **Rust-based** infrastructure-as-code (IaC) tool designed to provide a **high-level, descriptive, and user-friendly** way to define and manage Azure resources. It aims to be a **modern alternative** to Terraform and Bicep, offering a more intuitive syntax and seamless integration with Azure’s APIs.

🚀 **We’re looking for collaborators!** If you’re passionate about Rust, Azure, or IaC, we’d love your help. See the [Contributing](#contributing) section below.

---

## 🌟 Why AzureScript?

- **High-Level Abstractions**: Define Azure resources in a **descriptive, declarative YAML format**—no need to write low-level templates.
- **Rust-Powered**: Leverage Rust’s performance, safety, and concurrency for reliable infrastructure management.
- **Seamless Azure Integration**: Directly interact with Azure Resource Manager APIs for real-time resource management.
- **Extensible**: Easily add support for new Azure services or custom workflows.
- **Human-Readable**: Focus on clarity and simplicity, making it accessible for both developers and DevOps engineers.

---

## 📌 Features

- **YAML-Based Configuration**: Define your entire Azure infrastructure in a single, readable YAML file.
- **Azure API Integration**: Authenticate and interact with Azure APIs to create, update, and manage resources.
- **Modular Design**: Organize resources into subscriptions, resource groups, and services.
- **Error Handling**: Robust error handling for API calls, YAML parsing, and validation.
- **Open Source**: Fully open-source and community-driven.

---

## 🛠️ Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.60 or later)
- An [Azure account](https://azure.microsoft.com/)
- [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli) (for authentication)

### Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/wetcatsoftwarecompany/azurescript.git
   cd azurescript
Build the project:
bash Copycargo build --release

Run AzureScript:
bash Copy./target/release/azurescript --help



### 📂 Usage
1. Define Your Infrastructure:

    Create a YAML file (e.g., infra.yaml) to describe your Azure resources:
    ```yaml
    subscriptions:
    - name: "My Azure Subscription"
        id: "your-subscription-id"
        resource_groups:
        - name: "MyResourceGroup"
            region: "eastus"
            keyvaults:
            - name: "MyKeyVault"
                region: "eastus"
            virtual_machines:
            - name: "MyVM"
                size: "Standard_B1s"
                image: "UbuntuLTS"
    ```            
2. Authenticate with Azure:

    Obtain an Azure access token using the Azure CLI:
    ```bash
        az login
        az account get-access-token --query "accessToken" -o tsv
    ```
3. Deploy Your Infrastructure

    Run AzureScript to parse your configuration and interact with Azure:
    ```bash
        cargo run -- --config infra.yaml --access-token "your-access-token"
    ```

### 🤝 Contributing
We welcome contributions from the community! Whether you’re fixing bugs, adding features, improving documentation, or sharing ideas, your help is valuable.
How to Contribute

Fork the repository and clone it locally.
Create a new branch:
```bash
    git checkout -b my-feature-branch
```

Commit your changes:
```bash
    git commit -m "Add my feature"
```

Push to your fork:
```bash
    git push origin my-feature-branch
```

Open a Pull Request (PR) to the main branch.

Code of Conduct
Please follow our Code of Conduct in all interactions.

### 📜 License
AzureScript is licensed under the MIT License.

### 🔗 Links

GitHub Repository: https://github.com/wetcatsoftwarecompany/azurescript\
Issues: https://github.com/wetcatsoftwarecompany/azurescript/issues\
Pull Requests: https://github.com/wetcatsoftwarecompany/azurescript/pulls


### 📧 Contact
For questions, feedback, or collaboration, open an issue or reach out to the maintainers.

Thank you for using AzureScript! Let’s build the future of Azure IaC together. 🌟