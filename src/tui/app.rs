use crate::SeqeraClient;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    ComputeEnvs,
    Pipelines,
    Members,
    Settings,
}

impl Tab {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tab::ComputeEnvs => "Compute Envs",
            Tab::Pipelines => "Pipelines",
            Tab::Members => "Members",
            Tab::Settings => "Settings",
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeId {
    Organization(i64),
    Workspace(i64),
    ComputeEnvironment(String),
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub expanded: bool,
    pub children: Vec<TreeNode>,
    pub node_type: NodeType,
    pub id: Option<NodeId>,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Organization,
    Workspace,
    ComputeEnvironment,
}

pub struct App {
    pub title: String,
    pub should_quit: bool,
    pub show_help: bool,
    pub show_menu: bool,
    pub tabs: Vec<Tab>,
    pub current_tab: usize,
    pub tree: TreeNode,
    pub selected_path: Vec<usize>,
    pub status_message: String,
    pub client: SeqeraClient,
}

impl App {
    pub fn new(client: SeqeraClient) -> Self {
        Self {
            title: String::from("Seqera Console"),
            should_quit: false,
            show_help: false,
            show_menu: false,
            tabs: vec![Tab::ComputeEnvs, Tab::Pipelines, Tab::Members, Tab::Settings],
            current_tab: 0,
            tree: TreeNode {
                name: String::from("Organizations"),
                expanded: true,
                children: vec![],
                node_type: NodeType::Organization,
                id: None,
            },
            selected_path: vec![],
            status_message: String::from("Connected to api.cloud.seqera.io"),
            client,
        }
    }

    pub fn show_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub async fn refresh(&mut self) {
        self.status_message = "Refreshing...".to_string();
        match self.client.list_organizations().await {
            Ok(response) => {
                self.tree.children = response.organizations.into_iter().map(|org| TreeNode {
                    name: org.name,
                    expanded: false,
                    children: vec![],
                    node_type: NodeType::Organization,
                    id: Some(NodeId::Organization(org.id.0)),
                }).collect();
                self.status_message = "Connected to api.cloud.seqera.io".to_string();
            }
            Err(e) => {
                self.status_message = format!("Error: {}", e);
            }
        }
    }

    pub fn toggle_menu(&mut self) {
        self.show_menu = !self.show_menu;
    }

    pub fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % self.tabs.len();
    }

    pub fn previous_tab(&mut self) {
        if self.current_tab > 0 {
            self.current_tab -= 1;
        } else {
            self.current_tab = self.tabs.len() - 1;
        }
    }

    pub async fn handle_enter(&mut self) {
        let node_info = if let Some(node) = self.get_selected_node_mut() {
            Some((node.node_type.clone(), node.id.clone(), !node.expanded))
        } else {
            None
        };

        if let Some((node_type, id, should_expand)) = node_info {
            if should_expand {
                match node_type {
                    NodeType::Organization => {
                        if let Some(NodeId::Organization(org_id)) = id {
                            let result = self.client.list_workspaces(org_id).await;
                            if let Ok(response) = result {
                                let children = response.workspaces.into_iter().map(|ws| TreeNode {
                                    name: ws.name,
                                    expanded: false,
                                    children: vec![],
                                    node_type: NodeType::Workspace,
                                    id: Some(NodeId::Workspace(ws.id.0)),
                                }).collect();
                                
                                if let Some(node) = self.get_selected_node_mut() {
                                    node.children = children;
                                    node.expanded = true;
                                }
                                self.status_message = "Workspaces loaded".to_string();
                            } else if let Err(e) = result {
                                self.status_message = format!("Error loading workspaces: {}", e);
                            }
                        }
                    }
                    NodeType::Workspace => {
                        if let Some(NodeId::Workspace(workspace_id)) = id {
                            let result = self.client.list_compute_envs(workspace_id, None).await;
                            if let Ok(response) = result {
                                let children = response.compute_envs.into_iter().map(|ce| TreeNode {
                                    name: ce.name,
                                    expanded: false,
                                    children: vec![],
                                    node_type: NodeType::ComputeEnvironment,
                                    id: Some(NodeId::ComputeEnvironment(ce.id)),
                                }).collect();
                                
                                if let Some(node) = self.get_selected_node_mut() {
                                    node.children = children;
                                    node.expanded = true;
                                }
                                self.status_message = "Compute environments loaded".to_string();
                            } else if let Err(e) = result {
                                self.status_message = format!("Error loading compute environments: {}", e);
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                if let Some(node) = self.get_selected_node_mut() {
                    node.expanded = false;
                }
            }
        }
    }

    pub fn handle_left(&mut self) {
        if let Some(node) = self.get_selected_node_mut() {
            if node.expanded {
                node.expanded = false;
            } else if !self.selected_path.is_empty() {
                self.selected_path.pop();
            }
        }
    }

    pub fn handle_right(&mut self) {
        if let Some(node) = self.get_selected_node_mut() {
            if !node.expanded && !node.children.is_empty() {
                node.expanded = true;
            }
        }
    }

    pub fn handle_up(&mut self) {
        if self.selected_path.is_empty() {
            return;
        }

        let mut path = self.selected_path.clone();
        let mut found = false;

        // Try moving to previous sibling's deepest expanded child
        if let Some(_parent) = self.get_node_at_path(&path[..path.len()-1]) {
            let current_index = *path.last().unwrap();
            if current_index > 0 {
                // Move to previous sibling
                *path.last_mut().unwrap() = current_index - 1;
                
                // Keep going into the deepest expanded child
                let mut current = self.get_node_at_path(&path).unwrap();
                while current.expanded && !current.children.is_empty() {
                    path.push(current.children.len() - 1);
                    current = &current.children[current.children.len() - 1];
                }
                found = true;
            } else {
                // If we can't move to previous sibling, move up to parent
                path.pop();
                found = true;
            }
        }

        if found {
            self.selected_path = path;
        }
    }

    pub fn handle_down(&mut self) {
        // If we're at the root level
        if self.selected_path.is_empty() {
            if !self.tree.children.is_empty() {
                self.selected_path = vec![0];
            }
            return;
        }

        let mut current = &self.tree;
        let mut path = vec![];
        let mut found = false;

        // Traverse to current selection
        for &index in &self.selected_path {
            if index >= current.children.len() {
                return;
            }
            path.push(index);
            current = &current.children[index];

            // If this node is expanded and has children, we can potentially move into it
            if current.expanded && !current.children.is_empty() && path.len() == self.selected_path.len() {
                // Move to first child
                path.push(0);
                found = true;
                break;
            }
        }

        // If we haven't found a child to move to, try next sibling
        if !found {
            while !path.is_empty() {
                let parent = self.get_node_at_path(&path[..path.len()-1]);
                let current_index = *path.last().unwrap();

                if let Some(parent) = parent {
                    if current_index + 1 < parent.children.len() {
                        // Move to next sibling
                        *path.last_mut().unwrap() = current_index + 1;
                        found = true;
                        break;
                    }
                }
                // No more siblings at this level, move up one level
                path.pop();
            }
        }

        if found {
            self.selected_path = path;
        }
    }

    pub fn handle_escape(&mut self) {
        if self.show_help || self.show_menu {
            self.show_help = false;
            self.show_menu = false;
        } else if !self.selected_path.is_empty() {
            self.selected_path.pop();
        }
    }

    fn get_selected_node_mut(&mut self) -> Option<&mut TreeNode> {
        let mut current = &mut self.tree;
        for &index in &self.selected_path {
            if index >= current.children.len() {
                return None;
            }
            current = &mut current.children[index];
        }
        Some(current)
    }

    // fn get_current_parent_node(&self) -> Option<&TreeNode> {
    //     let mut current = &self.tree;
    //     if self.selected_path.is_empty() {
    //         return Some(current);
    //     }
    //     for (i, &index) in self.selected_path.iter().enumerate() {
    //         if i == self.selected_path.len() - 1 {
    //             return Some(current);
    //         }
    //         if index >= current.children.len() {
    //             return None;
    //         }
    //         current = &current.children[index];
    //     }
    //     Some(current)
    // }

    // Helper function to get node at a specific path
    fn get_node_at_path(&self, path: &[usize]) -> Option<&TreeNode> {
        let mut current = &self.tree;
        for &index in path {
            if index >= current.children.len() {
                return None;
            }
            current = &current.children[index];
        }
        Some(current)
    }
} 