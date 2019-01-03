use std::collections::BTreeSet;
use std::collections::HashSet;
use std::iter;

use crate::common::read_lines_from_file;

const RUNNING_TIME_OFFSET: usize = 60;
const NUM_WORKERS: usize = 5;

pub fn ch7() {
    let lines = read_lines_from_file("ch7.txt");
    let graph = Graph::parse(&lines).expect("Error during parsing a graph");
    let topologically_sorted_vertices = graph
        .topological_sort()
        .expect("Graph should not contain cycles");
    println!(
        "topological sort: {}",
        topologically_sorted_vertices
            .iter()
            .map(|v| v.c)
            .collect::<String>()
    );
    let jobs: Vec<_> = topologically_sorted_vertices
        .iter()
        .map(|v| Job::new(v, RUNNING_TIME_OFFSET))
        .collect();
    println!(
        "Running time for {} workers: {}",
        NUM_WORKERS,
        running_time(&jobs, NUM_WORKERS)
    );
}

#[derive(Debug)]
struct Graph {
    vertices_chars: Vec<char>,
    adj: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct Vertice {
    c: char,
    depends_on: Vec<char>,
}

impl Graph {
    fn parse(lines: &[String]) -> Result<Graph, String> {
        let mut edges = Vec::new();
        for l in lines {
            edges.push(Graph::parse_edge(l)?);
        }
        let vertices_chars: BTreeSet<_> = edges
            .iter()
            .flat_map(|(f, t)| iter::once(*f).chain(iter::once(*t)))
            .collect();
        // already sorted since BTreeSet
        let vertices_chars: Vec<_> = vertices_chars.into_iter().collect();

        let mut adj = vec![Vec::new(); vertices_chars.len()];
        for (f, t) in edges {
            let f = vertices_chars.iter().position(|c| *c == f).unwrap();
            let t = vertices_chars.iter().position(|c| *c == t).unwrap();
            adj[f].push(t);
        }
        for a in &mut adj {
            a.sort();
            a.reverse();
        }

        Ok(Graph {
            vertices_chars,
            adj,
        })
    }

    fn parse_edge(str: &str) -> Result<(char, char), String> {
        match scan_fmt!(
            str,
            "Step {} must be finished before step {} can begin.",
            char,
            char
        ) {
            (Some(f), Some(t)) => Ok((f, t)),
            _ => Err(format!("Unable to parse edge from {}", str)),
        }
    }

    fn topological_sort(&self) -> Result<Vec<Vertice>, &'static str> {
        let mut visited = vec![Visit::Not; self.vertices_chars.len()];
        let mut sorted = Vec::new();
        let mut dependencies = vec![BTreeSet::new(); self.vertices_chars.len()];

        for v in (0..self.vertices_chars.len()).rev() {
            self.visit(v, &mut visited, &mut sorted, &mut dependencies)?;
        }
        let sorted_char_vertices = sorted
            .iter()
            .rev()
            .map(|iv| Vertice {
                c: self.vertices_chars[*iv],
                depends_on: dependencies[*iv].iter().cloned().collect(),
            })
            .collect();
        Ok(sorted_char_vertices)
    }

    fn visit(
        &self,
        v: usize,
        visited: &mut Vec<Visit>,
        sorted: &mut Vec<usize>,
        dependencies: &mut Vec<BTreeSet<char>>,
    ) -> Result<(), &'static str> {
        match visited[v] {
            Visit::Permanent => Ok(()),
            Visit::Temporary => Err("Graph contains cycle, can't be topologically sorted"),
            Visit::Not => {
                visited[v] = Visit::Temporary;
                for n in &self.adj[v] {
                    dependencies[*n].insert(self.vertices_chars[v]);
                    self.visit(*n, visited, sorted, dependencies)?;
                }
                visited[v] = Visit::Permanent;
                sorted.push(v);
                Ok(())
            }
        }
    }
}

#[derive(Clone)]
enum Visit {
    Not,
    Permanent,
    Temporary,
}

#[derive(Clone, Debug)]
struct Job<'a> {
    c: char,
    depends_on: &'a Vec<char>,
    time: usize,
}

impl<'a> Job<'a> {
    fn new(v: &Vertice, time_offset: usize) -> Job {
        let time = ((v.c as u8) - b'A' + 1) as usize + time_offset;
        Job {
            c: v.c,
            depends_on: &v.depends_on,
            time,
        }
    }

    fn start_processing(&self) -> JobState {
        JobState::Running(self.c, self.time)
    }
}

#[derive(Clone)]
enum JobState {
    Running(char, usize),
    Completed(char),
}

impl JobState {
    fn time_passed(&self) -> JobState {
        match self {
            JobState::Running(c, 0) => JobState::Completed(*c),
            JobState::Running(c, time_left) => JobState::Running(*c, time_left - 1),
            _ => self.clone(),
        }
    }
}

struct JobQueue<'a> {
    jobs: Vec<Job<'a>>,
    jobs_count: usize,
    completed: HashSet<char>,
}

impl<'a> JobQueue<'a> {
    fn new(jobs_sorted: &[Job<'a>]) -> JobQueue<'a> {
        let jobs = jobs_sorted.iter().cloned().rev().collect();
        JobQueue {
            jobs,
            jobs_count: jobs_sorted.len(),
            completed: HashSet::new(),
        }
    }

    fn complete(&mut self, job_c: char) {
        self.completed.insert(job_c);
    }

    fn pop(&mut self) -> Option<JobState> {
        if self.jobs_count == self.completed.len() {
            return None;
        }

        let popped = self
            .jobs
            .iter()
            .enumerate()
            .rev()
            .find(|(_, j)| j.depends_on.iter().all(|dep| self.completed.contains(dep)))
            .map(|(i, j)| (i, j.start_processing()));

        match popped {
            Some((i, j)) => {
                self.jobs.remove(i);
                Some(j)
            }
            None => None,
        }
    }

    fn pop_and_process(&mut self) -> Option<JobState> {
        self.pop().map(|js| js.time_passed())
    }

    fn all_completed(&self) -> bool {
        self.completed.len() == self.jobs_count
    }
}

fn running_time(jobs: &[Job], num_workers: usize) -> usize {
    let mut workers_state = vec![None; num_workers];
    let mut job_queue = JobQueue::new(jobs);
    for ws in &mut workers_state {
        *ws = job_queue.pop_and_process();
    }
    let mut time = 0;
    while !job_queue.all_completed() {
        for ws in &mut workers_state {
            let new_state = match ws {
                Some(job_state) => Some(job_state.time_passed()),
                None => job_queue.pop_and_process(),
            };
            if let Some(JobState::Completed(c)) = new_state {
                job_queue.complete(c);
                *ws = job_queue.pop_and_process();
            } else {
                *ws = new_state;
            }
        }
        time += 1;
    }
    time
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"
        Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.
        "#;

    fn get_input_lines() -> Vec<String> {
        TEST_INPUT
            .split('\n')
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect()
    }

    #[test]
    fn test_topological_sort() {
        let graph = Graph::parse(&get_input_lines()).expect("Error during parsing a graph");
        assert_eq!(
            vec!['C', 'A', 'B', 'D', 'F', 'E'],
            graph
                .topological_sort()
                .unwrap()
                .iter()
                .map(|v| v.c)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_running_time_simple() {
        let vertices = vec![
            Vertice {
                c: 'D',
                depends_on: vec![],
            },
            Vertice {
                c: 'B',
                depends_on: vec!['D'],
            },
            Vertice {
                c: 'C',
                depends_on: vec!['D', 'B'],
            },
        ];
        let jobs: Vec<_> = vertices.iter().map(|v| Job::new(v, 0)).collect();
        assert_eq!(9, running_time(&jobs, 2));
    }

    #[test]
    fn test_running_time() {
        let graph = Graph::parse(&get_input_lines()).expect("Error during parsing a graph");
        let vertices_sorted = graph.topological_sort().unwrap();
        let jobs: Vec<_> = vertices_sorted.iter().map(|v| Job::new(v, 0)).collect();
        assert_eq!(15, running_time(&jobs, 2));
    }

}
