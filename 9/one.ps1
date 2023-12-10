function Is-ZeroRow {
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

function Get-NextValue {
    param (
        [int[]]$values
    )
    $next = [System.Collections.ArrayList]@()
    for ($i = 1; $i -le $values.Count; $i++) {
        $next.Add($values[$i] - $values[$i-1])
    }
    if(Is-ZeroRow -nums $next) {
        return 0
    }
    return 2
}

$data = Get-Content -Path .\example.txt
$arr = $data.Split("\n")
$total = 0;
foreach ($line in $arr) {
    $values = $line.Split(" ") | ForEach-Object { [int]$_ }
    #Write-Host $values
    $total += Get-NextValue -values $values
}

$total