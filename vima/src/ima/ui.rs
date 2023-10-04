use ima_core::{IMA, DebugModeProgram};
use ratatui::{
    prelude::{
        Backend,
        Rect, Layout, Direction, Constraint
    },
    Frame,
    widgets::{
        Paragraph,
        Borders,
        Block, Table, Row
    },
    style::{Style, Color}, text::{Text, Line, Span},
};

use crate::io::IO;

/// Split the given area into different zones for all widgets.
/// returns (debug_zone, ima_io_zone, program_zone, stack_zone, heap_zone, register_zone, energy_zone, flags_zone)
pub fn split_area(area: Rect) -> (Rect, Rect, Rect, Rect, Rect, Rect, Rect, Rect) {
    let main_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(4, 5),
                Constraint::Ratio(1, 5),
            ]
            .as_ref(),
        )
        .split(area);
    let machine_zone = main_split[0];
    let io_zone = main_split[1];
    
    let io_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(2, 7),
                Constraint::Ratio(5, 7),
            ]
            .as_ref(),
        )
        .split(io_zone);

    let machine_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(4, 18),
                Constraint::Ratio(5, 18),
                Constraint::Ratio(5, 18),
                Constraint::Ratio(4, 18),
            ]
            .as_ref(),
        )
        .split(machine_zone);

    let reg_flag_energy_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(10, 15),
                Constraint::Ratio(2, 15),
                Constraint::Ratio(3, 15),
            ]
            .as_ref(),
        )
        .split(machine_split[3]);

    (
        io_split[0],
        io_split[1],
        machine_split[0],
        machine_split[1],
        machine_split[2],
        reg_flag_energy_split[0],
        reg_flag_energy_split[1],
        reg_flag_energy_split[2],
    )
}


pub fn draw_io(frame: &mut Frame<impl Backend>, area: Rect, debug_io: &IO, focus: bool, title: &str) {

    let debug_size = area.height as usize - 2;

    let debug_lines = debug_io.display_stack(debug_size).collect::<Vec<_>>();

    let debug_cursor_height = debug_lines.len();
    
    let debug_io_par = Paragraph::new(debug_lines.join("\n"))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(title)
            .style(Style::default().fg(if focus {
                Color::LightGreen
            } else {
                Color::White
            }))
        );
    
    frame.render_widget(debug_io_par, area);

    if focus {
        frame.set_cursor(
            area.x + debug_io.cursor_pos() as u16 + 1,
            area.y + debug_cursor_height as u16,
        );
    }
}

pub fn draw_program(frame: &mut Frame<impl Backend>, area: Rect, ima: &IMA<DebugModeProgram>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Program");

    let available_space = area.height as u32 - 2;
    // somewhat it's nice to have the pc at the first third ?

    let mut program_start = ima.code.pc.saturating_sub(available_space / 3);
    let mut program_end = program_start + available_space.min(ima.code.code.0.len() as u32 - program_start);

    if program_end - program_start < available_space {
        // we have to center the program
        program_start = program_start.saturating_sub(available_space - (program_end - program_start));
        program_end = program_start + available_space.min(ima.code.code.0.len() as u32 - program_start);
    }

    let lines = ima.code.code.0[program_start as usize..program_end as usize].iter().enumerate().map(|(i, (line, bp))| {
        let line_n = i + program_start as usize;
        let pc = if ima.code.pc as usize == line_n { "PC>" } else { "   " };
        let bpc = if *bp { "⬤" } else { " " };
        let mut line_disp = Vec::new();
        for label in line.labels.iter() {
            line_disp.push(Span::styled(format!("{}: ", label.0), Style::default().fg(Color::Yellow)))
        }
        match &line.instruction {
            Some(instr) => line_disp.push(Span::styled(format!("{}", instr), Style::default().fg(Color::White))),
            None => {},
        }
        match &line.comment {
            Some(comment) => line_disp.push(Span::styled(format!(" ;{}", comment), Style::default().fg(Color::DarkGray))),
            None => {},
        }
        let mut spans = vec![
            Span::styled(format!("{line_n:<4}"), Style::default().fg(if *bp {Color::Red} else {Color::LightBlue})),
            Span::styled(format!("{pc}"), Style::default().fg(Color::LightGreen)),
            Span::styled(format!("{bpc}"), Style::default().fg(Color::Red)),
        ];
        spans.append(&mut line_disp);
        Line::from(spans)
    }).collect::<Vec<_>>();

    let program = Paragraph::new(lines).block(block);

    frame.render_widget(program, area);
}

pub fn draw_registers(frame: &mut Frame<impl Backend>, area: Rect, ima: &IMA<DebugModeProgram>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Registers");

    let registers = ima.registers.registers.iter().enumerate().map(|(i, d)| {
        let spans = vec![
            Span::styled(format!(" R{i:<2} : "), Style::default()),
            Span::styled(format!("{d}"), Style::default().fg(if d.is_undefined() { Color::DarkGray } else { Color::White })),
        ];
        Line::from(spans)
    }).collect::<Vec<_>>();

    let registers = Paragraph::new(registers).block(block);

    frame.render_widget(registers, area);
}

pub fn draw_stack(frame: &mut Frame<impl Backend>, area: Rect, ima: &IMA<DebugModeProgram>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Stack");

    let available_height = area.height as usize - 2;
    // let's try to position sp in the middle of the available space
    let mut stack_start = ima.sp.as_index().saturating_sub(available_height / 2);
    let mut stack_end = stack_start + available_height.min(ima.memory.stack.len() - stack_start);

    if stack_end - stack_start < available_height {
        // we have to center the stack
        stack_start = stack_start.saturating_sub(available_height - (stack_end - stack_start));
        stack_end = stack_start + available_height.min(ima.memory.stack.len() - stack_start);
    }

    let stack = ima.memory.stack[stack_start..stack_end].iter().enumerate().rev().map(|(i, d)| {
        let stack_n = i + stack_start;
        let sp = if ima.sp.as_index() == i + stack_start { "SP -> " } else { "      " };
        let gb_lb = match (ima.gb.as_index() == i + stack_start, ima.lb.as_index() == i + stack_start) {
            (true, true) => " <- GB, LB",
            (true, false) => " <- GB",
            (false, true) => " <- LB",
            (false, false) => "",
        };

        let spans = vec![
            Span::styled(format!("{sp}"), Style::default().fg(Color::LightGreen)),
            Span::styled(format!("{stack_n:<3}"), Style::default().fg(Color::LightBlue)),
            Span::styled(format!("| {d}"), Style::default().fg(if d.is_undefined() { Color::DarkGray } else { Color::White })),
            Span::styled(format!("{gb_lb}"), Style::default().fg(Color::LightGreen)),
        ];

        Line::from(spans)
    }).collect::<Vec<_>>();

    let stack = Paragraph::new(stack).block(block);

    frame.render_widget(stack, area);
}

pub fn draw_heap(frame: &mut Frame<impl Backend>, area: Rect, ima: &IMA<DebugModeProgram>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Heap");

    let allocations = ima.memory.allocator.allocations();
    let heap = allocations.into_iter().map(|(ptr, size)| {
        let mut spans = Vec::with_capacity(1 + size);
        spans.push(Line::styled(format!("Bloc {ptr} of size {size}"), Style::default().fg(Color::LightCyan)));

        for i in 0..size {
            match ima.memory.heap[ptr.as_index() + i] {
                Some(d) => spans.push(Line::from(vec![
                    Span::styled(format!(" {:<4}: ", ptr.as_index() + i), Style::default().fg(Color::LightBlue)),
                    Span::styled(format!(" {d}"), Style::default().fg(if d.is_undefined() { Color::DarkGray } else { Color::White }))
                ])),
                None => spans.push(Line::styled("Error - Please reapport this.", Style::default().fg(Color::Red))),
            }
        }

        spans.into_iter()
    }).flatten().collect::<Vec<_>>();

    let heap = Paragraph::new(heap).block(block);

    frame.render_widget(heap, area);
}

pub fn draw_energy(frame: &mut Frame<impl Backend>, area: Rect, ima: &IMA<DebugModeProgram>) {
    use ima_core::complete::CycleCost;

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Energy");

    let cycle_count = ima.cycle_count;
    let instr_cycle_cost = match ima.code.fetch() {
        Some(c) => c.cycle_cost(&ima.flags),
        None => 0,
    };

    let lines = vec![
        Line::from(vec![
            Span::styled(format!("Cycle count: "), Style::default().fg(Color::LightBlue)),
            Span::styled(format!("{cycle_count}"), Style::default()),
            Span::styled(format!(" + "), Style::default().fg(Color::LightBlue)),
            Span::styled(format!("{instr_cycle_cost}"), Style::default().fg(Color::DarkGray)),
        ])
    ];

    let energy = Paragraph::new(lines).block(block);

    frame.render_widget(energy, area);
}

pub fn draw_flags(frame: &mut Frame<impl Backend>, area: Rect, ima: &IMA<DebugModeProgram>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Flags");

    let flags = Table::new(
        vec![
            Row::new(vec![
                Text::styled(format!("EQ:{}", ima.flags.eq()), Style::default().fg(if ima.flags.eq() { Color::Green } else { Color::Red })),
                Text::styled(format!("GE:{}", ima.flags.ge()), Style::default().fg(if ima.flags.ge() { Color::Green } else { Color::Red })),
                Text::styled(format!("GT:{}", ima.flags.gt()), Style::default().fg(if ima.flags.gt() { Color::Green } else { Color::Red })),
            ]),
            Row::new(vec![
                Text::styled(format!("NE:{}", ima.flags.ne()), Style::default().fg(if ima.flags.ne() { Color::Green } else { Color::Red })),
                Text::styled(format!("LE:{}", ima.flags.le()), Style::default().fg(if ima.flags.le() { Color::Green } else { Color::Red })),
                Text::styled(format!("LT:{}", ima.flags.lt()), Style::default().fg(if ima.flags.lt() { Color::Green } else { Color::Red })),
            ]),
            Row::new(vec![
                Text::styled(format!("OV:{}", ima.flags.ov()), Style::default().fg(if ima.flags.ov() { Color::Green } else { Color::Red })),
            ]),
        ]
    )
    .block(block)
    .widths(&[Constraint::Ratio(1, 3); 3]);

    frame.render_widget(flags, area);
}