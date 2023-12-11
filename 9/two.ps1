function Is-ZeroRow {
    [OutputType([bool])]
    param (
        [int[]]$nums
    )
    foreach ($num in $nums) {
        <# $num is tnums current item #>
        if($num -ne 0) {
            return $false
        }
    }
    return $true
}

function Calculate-PrevElement {
    param (
        [int[]]$array
    )

    $nextArray = for ($i = 1; $i -lt $array.Count; $i++) {
        $array[$i] - $array[$i-1]
    }

    if ($nextArray.Count -le 1 -or (Is-ZeroRow -nums $nextArray)) {
        return $array[0]
    }
    return $array[0] - (Calculate-PrevElement -array $nextArray)
}

# Example usage:
$data = Get-Content -Path .\input.txt
$arr = $data -split "`n"

#Write-Host $arr

$total = 0
foreach ($line in $arr) {
    $values = $line.Split(" ") | ForEach-Object { [int]$_ }
    $nextElement = Calculate-PrevElement -array $values
    Write-Host $nextElement
    $total += $nextElement
}

Write-Host "Total of previous elements: " $total

