# Define a function to process each line
function next(c, curr, prev)
    #println("Processing line: ", line)
    # (-1,-1) (-1,0) (-1, 1)
    # ( 0,-1) ( 0,0) ( 0, 1)
    # ( 1,-1) ( 1,0) ( 1, 1)
    if c == '|'
        dir = curr[0] - prev[0]
        return (1,0) .* dir
    elseif c == '-'
        dir = curr[1] - prev[1]
        return (0,1) .* dir
    elseif c == 'L'
        dir = curr[0] - prev[0]
        return (0,1) .- dir
    elseif c == 'J'
        dir = curr[0] - prev[0]
        return (-1,0) + (dir, -dir)
    elseif c == '7'
        dir = curr[0] - prev[0]
        return (1,0) .+ dir
    elseif c == 'F'
        dir = curr[0] - prev[0]
        return (0,1) + (-dir, dir)
    else
        return (0,0)
    end
end

# Specify the path to your text file
file_path = "./example1.txt"

arr = Vector{Vector{Char}}()

# Open the file for reading
open(file_path, "r") do file
    # Read all lines from the file and store them in an array
    lines = readlines(file)
    push!(arr, repeat(['.'], length(lines[1]) + 2))

    # Iterate over each line
    for line in lines
        values = Vector{Char}()
        push!(values, '.')
        for c in line
            push!(values, c)
        end
        push!(values, '.')
        push!(arr, values)
        # Call the process_line function for each line
        #tra(line)
    end

    push!(arr, repeat(['.'], length(lines[1]) + 2))
end

for v in arr
    println(v)
end