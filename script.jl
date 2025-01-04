using SQLite
using DataFrames
using Statistics
using Distributions
using Plots
using LaTeXStrings

println("Hello World")

db = SQLite.DB("output.sqlite")
println("Connected to database")

function energy(db::SQLite.DB, run::Int)
    e_plt = plot()
    cv_plt = plot()

    for size in SQLite.DBInterface.execute(db, "SELECT size FROM configurations WHERE run_id = (?) GROUP BY size", [run]) |> DataFrame |> df -> Vector(df.size)
        e = Vector()
        e_std = Vector()
        e_sqr = Vector()
        e_sqr_std = Vector()
        temperatures = Vector()

        for temperature in SQLite.DBInterface.execute(db, "SELECT temperature FROM configurations WHERE run_id = (?) AND size = (?) ORDER BY temperature", [run, size]) |> DataFrame |> df -> Vector(df.temperature)
            data = SQLite.DBInterface.execute(db, "SELECT o.e FROM configurations AS c INNER JOIN observables AS o ON c.id = o.configuration_id WHERE c.run_id = (?) AND c.size = (?) AND c.temperature = (?) ORDER BY c.temperature, o.sequence_id", [current_run, size, temperature]) |> DataFrame
            data.e_sqr = data.e.^2
            data.group = repeat(1:Int(nrow(data) / 20), inner=20)

            binned = combine(groupby(data, :group), [:e, :e_sqr] .=> mean)
            
            tmp_e = Vector()
            tmp_e_sqr = Vector()

            for _ in 1:10_000
                push!(tmp_e, mean(sample(binned.e_mean, 1_000)))
                push!(tmp_e_sqr, mean(sample(binned.e_sqr_mean, 1_000)))
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

        scatter!(e_plt, res.t, res.e, yerror=res.e_std, ms=1, msc=:auto, label=latexstring("N = $(size)"))
        scatter!(cv_plt, res.t, res.cv, ms=1, msc=:auto, label=latexstring("N = $(size)"))
        println()
    end

    savefig(e_plt, "energy.pdf")
    savefig(cv_plt, "specific_heat.pdf")
end


current_run = SQLite.DBInterface.execute(db, "SELECT id FROM runs ORDER BY created_at DESC") |> DataFrame |> df -> df.id[1]
energy(db, current_run)

