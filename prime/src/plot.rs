use plotly::common::Mode;
use plotly::{Layout, Plot, Scatter};

pub fn plot_spiral(t: &[f64]) {
    let mut plot = Plot::new();

    let layout = Layout::new().title("Primes").width(1000).height(1000);
    plot.set_layout(layout);

    let xs = t.iter().map(|&x| x * x.sin()).collect::<Vec<f64>>();
    let ys = t.iter().map(|&x| x * x.cos()).collect::<Vec<f64>>();
    let trace = Scatter::new(xs, ys).mode(Mode::Markers);
    plot.add_trace(trace);

    let line_trace_100k = Scatter::new(vec![0.0, -1_802.649], vec![0.0, 99_672.70]);
    plot.add_trace(line_trace_100k);

    let line_trace_200k = Scatter::new(vec![0.0, -1_914.645], vec![0.0, 199_789.80]);
    plot.add_trace(line_trace_200k);

    plot.write_html("prime_spiral.html");
}
