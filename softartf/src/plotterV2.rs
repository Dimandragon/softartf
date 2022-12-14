use std::collections::HashMap;
use std::fmt::Display;
use plotters::prelude::*;
use crate::Complex;

pub fn plot_lines<P, S>(data: &Vec<&Vec<f32>>, path: &P, (w, h): (u32, u32), name: Option<S>, names_lines: Option<Vec<&S>>)
    where P: AsRef<std::path::Path> + ?Sized, S: AsRef<str> + Clone, String: From<S> {
    let maximum = crate::math::instrumental_math::max_double_arr(data);
    let minimum = crate::math::instrumental_math::min_double_arr(data);
    let lenght = crate::math::instrumental_math::max_len(data);

    if maximum.is_some() && minimum.is_some() && lenght.is_some(){
        let root_area = BitMapBackend::new(path, (w, h))
            .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if name.is_some(){
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d(0.0..(lenght.unwrap() as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32) + 1.0)
                .unwrap();

            if names_lines.is_some(){
                assert!((names_lines.as_ref().unwrap().len()==data.len()));
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let color = Palette99::pick(id_series).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, *x as f32)), color))
                        .unwrap()
                        .label(Clone::clone(names_lines.as_ref().unwrap()[id_series]))
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
            else{
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let mut name_series: String = String::new();
                    name_series.push_str("data");
                    name_series.push_str(& format!("{}", id_series));
                    let color = Palette99::pick(id_series).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, *x as f32)), color))
                        .unwrap()
                        .label(name_series)
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
        }
        else {
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(0.0..(lenght.unwrap() as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32 + 1.0))
                .unwrap();

            if names_lines.is_some(){
                assert!((names_lines.as_ref().unwrap().len()==data.len()));
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let color = Palette99::pick(id_series).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, *x as f32)), color))
                        .unwrap()
                        .label(Clone::clone(names_lines.as_ref().unwrap()[id_series]))
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
            else{
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let mut name_series: String = String::new();
                    name_series.push_str("data");
                    name_series.push_str(& format!("{}", id_series));
                    let color = Palette99::pick(id_series).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, *x as f32)), color))
                        .unwrap()
                        .label(name_series)
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
        }
    }
}

pub fn plot_lines_complex<P, S>(data: &Vec<&Vec<Complex<f32>>>, path: &P, (w, h): (u32, u32), name: Option<S>, names_lines: Option<Vec<&S>>)
    where P: AsRef<std::path::Path> + ?Sized, S: AsRef<str> + Clone, String: From<S> {
    let maximum = crate::math::instrumental_math::max_double_arr_complex(data);
    let minimum = crate::math::instrumental_math::min_double_arr_complex(data);
    let lenght = crate::math::instrumental_math::max_len(data);

    if maximum.is_some() && minimum.is_some() && lenght.is_some(){
        let root_area = BitMapBackend::new(path, (w, h))
            .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if name.is_some(){
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d(0.0..(lenght.unwrap() as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32 + 1.0))
                .unwrap();

            if names_lines.is_some(){
                assert!((names_lines.as_ref().unwrap().len()==data.len()));
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let color1 = Palette99::pick(id_series*2).mix(0.9);
                    let mut name1: String = Clone::clone(names_lines.as_ref().unwrap()[id_series]).into();
                    name1.push_str(".real");

                    for i in 0..series.len(){
                        if series[i].re.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.re)), color1))
                        .unwrap()
                        .label(Clone::clone(&name1))
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color1));

                    let color2 = Palette99::pick(id_series*2+1).mix(0.9);
                    let mut name2: String = Clone::clone(names_lines.as_ref().unwrap()[id_series]).into();
                    name2.push_str(".image");

                    for i in 0..series.len(){
                        if series[i].im.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.im)), color2))
                        .unwrap()
                        .label(Clone::clone(&name2))
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color2));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
            else{
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let mut name_series: String = String::new();
                    name_series.push_str("data");
                    name_series.push_str(& format!("{}", id_series));
                    name_series.push_str(".real");
                    let color = Palette99::pick(id_series*2).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].re.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.re)), color))
                        .unwrap()
                        .label(name_series)
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

                    let mut name_series2: String = String::new();
                    name_series2.push_str("data");
                    name_series2.push_str(& format!("{}", id_series));
                    name_series2.push_str(".image");
                    let color2 = Palette99::pick(id_series*2).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].im.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.im)), color2))
                        .unwrap()
                        .label(name_series2)
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color2));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
        }
        else {
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(0.0..(lenght.unwrap() as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32 + 1.0))
                .unwrap();

            if names_lines.is_some(){
                assert!((names_lines.as_ref().unwrap().len()==data.len()));
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let color1 = Palette99::pick(id_series*2).mix(0.9);
                    let mut name1: String = Clone::clone(names_lines.as_ref().unwrap()[id_series]).into();
                    name1.push_str(".real");

                    for i in 0..series.len(){
                        if series[i].re.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.re)), color1))
                        .unwrap()
                        .label(Clone::clone(&name1))
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color1));

                    let color2 = Palette99::pick(id_series*2+1).mix(0.9);
                    let mut name2: String = Clone::clone(names_lines.as_ref().unwrap()[id_series]).into();
                    name2.push_str(".image");

                    for i in 0..series.len(){
                        if series[i].im.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.im)), color2))
                        .unwrap()
                        .label(Clone::clone(&name2))
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color2));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
            else{
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let mut name_series: String = String::new();
                    name_series.push_str("data");
                    name_series.push_str(& format!("{}", id_series));
                    name_series.push_str(".real");
                    let color = Palette99::pick(id_series*2).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].re.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.re)), color))
                        .unwrap()
                        .label(name_series)
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

                    let mut name_series2: String = String::new();
                    name_series2.push_str("data");
                    name_series2.push_str(& format!("{}", id_series));
                    name_series2.push_str(".image");
                    let color2 = Palette99::pick(id_series*2).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].im.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.im)), color2))
                        .unwrap()
                        .label(name_series2)
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color2));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
        }
    }
}

pub fn plot_lines_complex_ignore<P, S>(data: &Vec<&Vec<Complex<f32>>>, path: &P, (w, h): (u32, u32), name: Option<S>, names_lines: Option<Vec<&S>>)
    where P: AsRef<std::path::Path> + ?Sized, S: AsRef<str> + Clone, String: From<S> {
    let maximum = crate::math::instrumental_math::max_double_arr_complex_ignore(data);
    let minimum = crate::math::instrumental_math::min_double_arr_complex_ignore(data);
    let lenght = crate::math::instrumental_math::max_len(data);

    if maximum.is_some() && minimum.is_some() && lenght.is_some(){
        let root_area = BitMapBackend::new(path, (w, h))
            .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if name.is_some(){
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d(0.0..(lenght.unwrap() as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32+ 1.0))
                .unwrap();

            if names_lines.is_some(){
                assert!((names_lines.as_ref().unwrap().len()==data.len()));
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let color = Palette99::pick(id_series).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].re.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.re)), color))
                        .unwrap()
                        .label(Clone::clone(names_lines.as_ref().unwrap()[id_series]))
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
            else{
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let mut name_series: String = String::new();
                    name_series.push_str("data");
                    name_series.push_str(& format!("{}", id_series));
                    let color = Palette99::pick(id_series).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].re.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                        if series[i].re>maximum.unwrap() as f32{
                            //println!("gfdgazdgsg");
                            //println!("{} {}", series[i].re, maximum.unwrap());
                        }
                        if series[i].re<minimum.unwrap() as f32{
                            //println!("gtrsehesjytd");
                            //println!("{} {}", series[i].re, minimum.unwrap());
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.re)), color))
                        .unwrap()
                        .label(name_series)
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
        }
        else {
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(0.0..(lenght.unwrap() as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32 + 1.0))
                .unwrap();

            if names_lines.is_some(){
                assert!((names_lines.as_ref().unwrap().len()==data.len()));
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let color = Palette99::pick(id_series).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].re.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.re)), color))
                        .unwrap()
                        .label(Clone::clone(names_lines.as_ref().unwrap()[id_series]))
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
            else{
                ctx.configure_mesh().draw().unwrap();

                for (id_series, &series) in data.
                    iter().
                    enumerate()
                {
                    let mut name_series: String = String::new();
                    name_series.push_str("data");
                    name_series.push_str(& format!("{}", id_series));
                    let color = Palette99::pick(id_series).mix(0.9);

                    for i in 0..series.len(){
                        if series[i].re.is_finite()==false{
                            //println!("ASDfASdfasd");
                        }
                    }

                    ctx.draw_series(LineSeries::new(series
                                                        .iter()
                                                        .enumerate()
                                                        .map(|(idx, x)| (idx as f32, x.re)), color))
                        .unwrap()
                        .label(name_series)
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
                }
                ctx.configure_series_labels()
                    .border_style(&BLACK)
                    .background_style(&WHITE.mix(0.6))
                    .draw()
                    .unwrap();
            }
        }
    }
}

pub fn single_plot<P, S>(data: &Vec<f32>, path: &P, name: Option<S>, (w, h): (u32, u32))
    where P: AsRef<std::path::Path> + ?Sized, S: AsRef<str> + Clone, String: From<S>
{
    let maximum = crate::math::instrumental_math::max(data);
    let minimum = crate::math::instrumental_math::min(data);
    let lenght = data.len();

    if maximum.is_some() && minimum.is_some(){
        let root_area = BitMapBackend::new(path, (w, h))
            .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if name.is_some(){
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d(0.0..(lenght as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32) + 1.0)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            let color = Palette99::pick(0).mix(0.9);

            for i in 0..data.len(){
                if data[i].is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }

            ctx.draw_series(LineSeries::new(data
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, x)| (idx as f32, *x as f32)), color))
                                                .unwrap();
        }
        else {
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(0.0..(lenght as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32) + 1.0)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            let color = Palette99::pick(0).mix(0.9);

            for i in 0..data.len(){
                if data[i].is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }

            ctx.draw_series(LineSeries::new(data
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, x)| (idx as f32, *x as f32)), color))
                                                .unwrap();
        }
    }
}

pub fn single_complex_ignore_plot<P, S>(data: &Vec<Complex<f32>>, path: &P, name: Option<S>, (w, h): (u32, u32))
    where P: AsRef<std::path::Path> + ?Sized, S: AsRef<str> + Clone, String: From<S>
{
    let maximum = crate::math::instrumental_math::max_complex_ignore(data);
    let minimum = crate::math::instrumental_math::min_complex_ignore(data);
    let lenght = data.len();

    if maximum.is_some() && minimum.is_some(){
        let root_area = BitMapBackend::new(path, (w, h))
            .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if name.is_some(){
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d(0.0..(lenght as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32) + 1.0)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            let color = Palette99::pick(0).mix(0.9);

            for i in 0..data.len(){
                if data[i].re.is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }

            ctx.draw_series(LineSeries::new(data
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, x)| (idx as f32, x.re)), color))
                                                .unwrap();
        }
        else {
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(0.0..(lenght as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32) + 1.0)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            let color = Palette99::pick(0).mix(0.9);

            for i in 0..data.len(){
                if data[i].re.is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }

            ctx.draw_series(LineSeries::new(data
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, x)| (idx as f32, x.re)), color))
                                                .unwrap();
        }
    }
}

pub fn single_complex_plot<P, S>(data: &Vec<Complex<f32>>, path: &P, name: Option<S>, (w, h): (u32, u32))
    where P: AsRef<std::path::Path> + ?Sized, S: AsRef<str> + Clone, String: From<S>
{
    let maximum = crate::math::instrumental_math::max_complex(data);
    let minimum = crate::math::instrumental_math::min_complex(data);
    let lenght = data.len();

    if maximum.is_some() && minimum.is_some(){
        let root_area = BitMapBackend::new(path, (w, h))
            .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if name.is_some(){
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d(0.0..(lenght as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32) + 1.0)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            let color = Palette99::pick(0).mix(0.9);

            for i in 0..data.len(){
                if data[i].re.is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }
            let name = "real";

            ctx.draw_series(LineSeries::new(data
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, x)| (idx as f32, x.re)), color))
                .unwrap()
                .label(Clone::clone(&name))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

            let color = Palette99::pick(1).mix(0.9);

            for i in 0..data.len(){
                if data[i].im.is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }

            let name2 = "image";
            ctx.draw_series(LineSeries::new(data
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, x)| (idx as f32, x.im)), color))
                .unwrap()
                .label(Clone::clone(&name2))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

            ctx.configure_series_labels()
                .border_style(&BLACK)
                .background_style(&WHITE.mix(0.6))
                .draw()
                .unwrap();
        }
        else {
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(0.0..(lenght as f32 + 1.0), (minimum.unwrap() as f32 + 1.0)..(maximum.unwrap() as f32) + 1.0)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            let color = Palette99::pick(0).mix(0.9);

            for i in 0..data.len(){
                if data[i].re.is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }
            let name = "real";

            ctx.draw_series(LineSeries::new(data
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, x)| (idx as f32, x.re)), color))
                                                .unwrap()
                                                .label(Clone::clone(&name))
                                                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

            let color = Palette99::pick(1).mix(0.9);

            for i in 0..data.len(){
                if data[i].im.is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }

            let name2 = "image";
            ctx.draw_series(LineSeries::new(data
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, x)| (idx as f32, x.im)), color))
                                                .unwrap()
                                                .label(Clone::clone(&name2))
                                                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

            ctx.configure_series_labels()
                .border_style(&BLACK)
                .background_style(&WHITE.mix(0.6))
                .draw()
                .unwrap();
        }
    }
}

pub fn freq_repr_vec_plot<P, S>(data: &Vec<f32>, path: &P, name: Option<S>, (w, h): (u32, u32))
    where P: AsRef<std::path::Path> + ?Sized, S: AsRef<str> + Clone, String: From<S>
{
    let maximum = crate::math::instrumental_math::max(data);
    let minimum = crate::math::instrumental_math::min(data);
    let lenght = data.len();

    let root_area = BitMapBackend::new(path, (w, h))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    if maximum.is_some() && minimum.is_some(){
        if name.is_some(){
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d((0..lenght as i32 + 1).into_segmented(), (minimum.unwrap() as i32 + 1)..(maximum.unwrap() as i32 + 1))
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            for i in 0..data.len(){
                if data[i].is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }

            ctx.draw_series(data
                .iter()
                .enumerate()
                .map(|(x, y)| {
                    let x0 = SegmentValue::Exact(x as i32);
                    let x1 = SegmentValue::Exact(x as i32 + 1);
                    let mut bar = Rectangle::new([(x0, 0), (x1, *y as i32)], RED.filled());
                    bar
                }))
                .unwrap();
        }
        else {
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d((0..lenght as i32 + 1).into_segmented(), (minimum.unwrap() as i32 + 1)..(maximum.unwrap() as i32) + 1)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            for i in 0..data.len(){
                if data[i].is_finite()==false{
                    //println!("ASDfASdfasd");
                }
            }

            ctx.draw_series(data
                .iter()
                .enumerate()
                .map(|(x, y)| {
                    let x0 = SegmentValue::Exact(x as i32);
                    let x1 = SegmentValue::Exact(x as i32 + 1);
                    let mut bar = Rectangle::new([(x0, 0), (x1, *y as i32)], RED.filled());
                    bar
                }))
                .unwrap();
        }
    }
}

pub fn freq_repr_map_plot<P, S>(data: &HashMap<i32, f32>, path: &P, name: Option<S>, (w, h): (u32, u32))
    where P: AsRef<std::path::Path> + ?Sized, S: AsRef<str> + Clone, String: From<S>
{
    let minimum = crate::math::instrumental_math::min_map(data);
    let maximum = crate::math::instrumental_math::max_map(data);

    //println!("{} {} {} {}", minimum.unwrap().0, maximum.unwrap().0, minimum.unwrap().1, maximum.unwrap().1);

    let root_area = BitMapBackend::new(path, (w, h))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    if maximum.is_some() && minimum.is_some(){
        if name.is_some(){
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d(((minimum.unwrap().0 as i32 - 1)..(maximum.unwrap().0 as i32 +1)).into_segmented()
                                    ,(minimum.unwrap().1 as i32 - 1)..(maximum.unwrap().1 as i32 + 1))
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            ctx.draw_series(data
                .iter()
                .map(|(x, y)| {
                    let x0 = SegmentValue::Exact(*x as i32);
                    let x1 = SegmentValue::Exact(*x as i32 + 1);
                    let mut bar = Rectangle::new([(x0, 0), (x1, *y as i32)], RED.filled());
                    bar
                }))
                .unwrap();
        }
        else{
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(((minimum.unwrap().0 as i32 - 1)..maximum.unwrap().0 as i32 +1).into_segmented()
                                    ,(minimum.unwrap().1 as i32 - 1)..(maximum.unwrap().1 as i32) + 1)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            ctx.draw_series(data
                .iter()
                .map(|(x, y)| {
                    let x0 = SegmentValue::Exact(*x as i32);
                    let x1 = SegmentValue::Exact(*x as i32 + 1);
                    let mut bar = Rectangle::new([(x0, 0), (x1, *y as i32)], RED.filled());
                    bar
                }))
                .unwrap();
        }
    }

}

pub fn get_colour<T>(colour_min: RGBColor, colour_max: RGBColor, max: T, min: T, value: T, pow_base: f64, log_base: f64)->RGBColor
where T: Into<f64> + Copy + PartialOrd + Display
{
    if value<min{
        println!("{} {}", value, min);
    }
    assert!(value>=min);
    if value>max{
        println!("{} {}", value, max);
    }
    assert!(value<=max);

    let r = (Into::<f64>::into(colour_min.0) + ((Into::<f64>::into(colour_max.0) - Into::<f64>::into(colour_min.0)) * (Into::<f64>::into(value).powf(pow_base).log(log_base) - Into::<f64>::into(min).powf(pow_base).log(log_base)) / (Into::<f64>::into(max).powf(pow_base).log(log_base) - Into::<f64>::into(min).powf(pow_base).log(log_base)))) as u8;
    let g = (Into::<f64>::into(colour_min.1) + ((Into::<f64>::into(colour_max.1) - Into::<f64>::into(colour_min.1)) * (Into::<f64>::into(value).powf(pow_base).log(log_base) - Into::<f64>::into(min).powf(pow_base).log(log_base)) / (Into::<f64>::into(max).powf(pow_base).log(log_base) - Into::<f64>::into(min).powf(pow_base).log(log_base)))) as u8;
    let b = (Into::<f64>::into(colour_min.2) + ((Into::<f64>::into(colour_max.2) - Into::<f64>::into(colour_min.2)) * (Into::<f64>::into(value).powf(pow_base).log(log_base) - Into::<f64>::into(min).powf(pow_base).log(log_base)) / (Into::<f64>::into(max).powf(pow_base).log(log_base) - Into::<f64>::into(min).powf(pow_base).log(log_base)))) as u8;

    RGBColor{ 0: r, 1: g, 2: b }
}

pub fn spectrogramm<P, S>(data: &Vec<HashMap<i32, f32>>, path: &P, name: Option<S>, (w, h): (u32, u32), colour_min: RGBColor, colour_max: RGBColor, pow_base: f64, log_base: f64)
    where P: AsRef<std::path::Path> + ?Sized, S: AsRef<str> + Clone, String: From<S>
{
    let minimum = crate::math::instrumental_math::min_vec_map(data);
    let maximum = crate::math::instrumental_math::max_vec_map(data);

    if maximum.is_some() && minimum.is_some(){
        println!("{} {}", minimum.unwrap().0, minimum.unwrap().1);
        println!("{} {}", maximum.unwrap().0, maximum.unwrap().1);

        let root_area = BitMapBackend::new(path, (w, h))
            .into_drawing_area();
        root_area.fill(&colour_min).unwrap();

        if name.is_some(){
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d((0i32..(data.len() as i32)).into_segmented(),
                    ((minimum.unwrap().0 - 1)..(maximum.unwrap().0 + 1)).into_segmented())
                .unwrap();


            //data.iter().enumerate().map(|(x, hash_map)|
            for (x, hash_map) in data.iter().enumerate()
                {
                ctx.draw_series(hash_map.iter().map(|(y, value)|{
                    let colour = get_colour(colour_min, colour_max, maximum.unwrap().1, minimum.unwrap().1, *value, pow_base, log_base);
                    Rectangle::new([(SegmentValue::Exact(x as i32), SegmentValue::Exact(*y)), (SegmentValue::Exact(x as i32 +1), SegmentValue::Exact(*y+1))], ShapeStyle{color: colour.to_rgba(), filled: true, stroke_width: 0})
                })).unwrap();
            }

            /*ctx.draw_series(data
                .iter()
                .enumerate()
                .map(|(x, map)| {
                    map.iter().map((|(y, value)|{
                        let colour = get_colour(colour_min, colour_max, maximum.unwrap().1, minimum.unwrap().1, *value);
                        Rectangle::new([((x as i32), *y), ((x as i32 +1), *y+1)], colour)
                    }))
                })).unwrap();*/
        }
        else {
            let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(name.unwrap(), ("Arial", 40))
                .build_cartesian_2d((0i32..(data.len() as i32)).into_segmented(),
                                    ((minimum.unwrap().0 - 1)..(maximum.unwrap().0 + 1)).into_segmented())
                .unwrap();

            for (x, hash_map) in data.iter().enumerate()
            {
                ctx.draw_series(hash_map.iter().map(|(y, value)|{
                    let colour = get_colour(colour_min, colour_max, maximum.unwrap().1, minimum.unwrap().1, *value, pow_base, log_base);
                    Rectangle::new([(SegmentValue::Exact(x as i32), SegmentValue::Exact(*y)), (SegmentValue::Exact(x as i32 +1), SegmentValue::Exact(*y+1))], ShapeStyle{color: colour.to_rgba(), filled: true, stroke_width: 0})
                })).unwrap();
            }
        }
    }
}