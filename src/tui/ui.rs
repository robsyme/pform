use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, List, ListItem, ListState},
    Frame,
};

use super::app::{App, Tab, TreeNode, NodeType};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Title bar
            Constraint::Min(0),     // Main content
            Constraint::Length(1),  // Status bar
        ])
        .split(f.size());

    draw_title_bar(f, app, chunks[0]);
    draw_main_content(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);
}

fn draw_title_bar(f: &mut Frame, app: &App, area: Rect) {
    let title = vec![
        Span::raw(app.title.clone()),
        Span::raw(" | "),
        Span::styled("[F1] Help", Style::default().fg(Color::Gray)),
        Span::raw(" "),
        Span::styled("[F5] Refresh", Style::default().fg(Color::Gray)),
        Span::raw(" "),
        Span::styled("[F10] Menu", Style::default().fg(Color::Gray)),
        Span::raw(" "),
        Span::styled("[Q] Quit", Style::default().fg(Color::Gray)),
    ];
    let paragraph = Paragraph::new(Line::from(title))
        .style(Style::default().bg(Color::Blue));
    f.render_widget(paragraph, area);
}

fn draw_main_content(f: &mut Frame, app: &App, area: Rect) {
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

    draw_tree(f, app, horizontal_chunks[0]);
    draw_details(f, app, horizontal_chunks[1]);
}

fn draw_tree(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Navigation")
        .borders(Borders::ALL);
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let mut items = Vec::new();
    render_tree_node(&app.tree, 0, &mut items);

    let selected_index = if app.selected_path.is_empty() {
        0
    } else {
        // Calculate the absolute index in the flattened list
        let mut index = 0;
        let mut current = &app.tree;
        for (i, &path_index) in app.selected_path.iter().enumerate() {
            // Add indices for all visible items before this level
            for j in 0..path_index {
                index += 1;
                if current.children[j].expanded {
                    index += count_visible_children(&current.children[j]);
                }
            }
            // Add one for the current item
            index += 1;
            if i < app.selected_path.len() - 1 {
                current = &current.children[path_index];
            }
        }
        index
    };

    let list = List::new(items)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("→ ");
    f.render_stateful_widget(list, inner_area, &mut ListState::default().with_selected(Some(selected_index)));
}

fn render_tree_node(node: &TreeNode, depth: usize, items: &mut Vec<ListItem>) {
    let prefix = "  ".repeat(depth);
    let indicator = if node.children.is_empty() {
        "  "
    } else if node.expanded {
        "[-]"
    } else {
        "[+]"
    };

    let style = match node.node_type {
        NodeType::Organization => Style::default().fg(Color::Green),
        NodeType::Workspace => Style::default().fg(Color::Yellow),
        NodeType::ComputeEnvironment => Style::default().fg(Color::Cyan),
    };

    items.push(ListItem::new(Line::from(vec![
        Span::raw(format!("{}{} {}", prefix, indicator, node.name)),
    ]).style(style)));

    if node.expanded {
        for child in &node.children {
            render_tree_node(child, depth + 1, items);
        }
    }
}

fn count_visible_children(node: &TreeNode) -> usize {
    let mut count = node.children.len();
    if node.expanded {
        for child in &node.children {
            if child.expanded {
                count += count_visible_children(child);
            }
        }
    }
    count
}

fn draw_details(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
        ])
        .split(area);

    let titles = app.tabs.iter().map(|t| t.as_str()).collect::<Vec<_>>();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL))
        .select(app.current_tab)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_widget(tabs, chunks[0]);

    // Render tab content based on current_tab
    match app.tabs[app.current_tab] {
        Tab::ComputeEnvs => draw_compute_envs(f, app, chunks[1]),
        Tab::Pipelines => draw_pipelines(f, app, chunks[1]),
        Tab::Members => draw_members(f, app, chunks[1]),
        Tab::Settings => draw_settings(f, app, chunks[1]),
    }
}

fn draw_compute_envs(f: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .title("Compute Environments")
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

fn draw_pipelines(f: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .title("Pipelines")
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

fn draw_members(f: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .title("Members")
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

fn draw_settings(f: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .title("Settings")
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status = Paragraph::new(app.status_message.as_str())
        .style(Style::default().bg(Color::Blue));
    f.render_widget(status, area);
} 