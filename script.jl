using SQLite
using DataFrames
using Statistics
using Distributions
using Plots
using LaTeXStrings
using LsqFit

db = SQLite.DB("output.sqlite")
println("Connected to database")

function energy(db::SQLite.DB, run::Int)
    e_plt = plot()
    cv_plt = plot()
    scaling_plot = plot()

    sizes = Vector()
    maximums = Vector()
    maximums_std = Vector()


    for size in SQLite.DBInterface.execute(db, "SELECT size FROM configurations WHERE run_id = (?) GROUP BY size", [run]) |> DataFrame |> df -> Vector(df.size)
        e = Vector()
        e_std = Vector()
        e_sqr = Vector()
        e_sqr_std = Vector()
        temperatures = Vector()

        for temperature in SQLite.DBInterface.execute(db, "SELECT temperature FROM configurations WHERE run_id = (?) AND size = (?) ORDER BY temperature", [run, size]) |> DataFrame |> df -> Vector(df.temperature)
            data = SQLite.DBInterface.execute(db, "SELECT o.m AS e FROM configurations AS c INNER JOIN observables AS o ON c.id = o.configuration_id WHERE c.run_id = (?) AND c.size = (?) AND c.temperature = (?) AND o.sequence_id >= 1000 ORDER BY c.temperature, o.sequence_id", [current_run, size, temperature]) |> DataFrame
            data.e_sqr = data.e.^2
            data.group = repeat(1:Int(nrow(data) / 10), inner=10)

            binned = combine(groupby(data, :group), [:e, :e_sqr] .=> mean)
            
            tmp_e = Vector()
            tmp_e_sqr = Vector()

            for _ in 1:1_000
                push!(tmp_e, mean(sample(binned.e_mean, nrow(binned), replace=true)))
                push!(tmp_e_sqr, mean(sample(binned.e_sqr_mean, nrow(binned), replace=true)))
            end

            push!(temperatures, temperature)
            push!(e, mean(tmp_e))
            push!(e_std, std(tmp_e))
            push!(e_sqr, mean(tmp_e_sqr))
            push!(e_sqr_std, std(tmp_e_sqr))

            print("\r\tPlotting $(size): $(temperature)")
        end

        res = DataFrame(t=temperatures, e=e, e_std=e_std, e_sqr=e_sqr, e_sqr_std=e_sqr_std)
        res.cv = (res.e_sqr - res.e.^2) ./ res.t
        res.cv_std = sqrt.((res.e_sqr_std ./ res.t).^2 + (2 .* res.e .* res.e_std ./ res.t).^2)

        _, idx = findmax(res.cv)
        push!(sizes, size)
        push!(maximums, res[idx, :].t)
        push!(maximums_std, 0.01)

        scatter!(e_plt, res.t, res.e, yerror=res.e_std, ms=1, msc=:auto, label=latexstring("N = $(size)"))
        scatter!(cv_plt, res.t, res.cv, yerror=res.cv_std, ms=1, msc=:auto, label=latexstring("N = $(size)"))
        println()
    end

    scatter!(scaling_plot, 1 ./ log.(sizes).^2, maximums, yerror=maximums_std, ms=1, msc=:auto)

    model(t, p) = p[1] * t .+ p[2]
    fit = curve_fit(model, 1 ./ log.(sizes).^2, maximums, [0.01, 0.88])
    println("Params: $(fit.param)")


    savefig(e_plt, "energy.pdf")
    savefig(cv_plt, "specific_heat.pdf")
    savefig(scaling_plot, "scaling.pdf")
end


current_run = SQLite.DBInterface.execute(db, "SELECT id FROM runs ORDER BY created_at DESC") |> DataFrame |> df -> df.id[1]
energy(db, current_run)

