
use builtin;
use str;

set edit:completion:arg-completer[mc] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'mc'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'mc'= {
            cand -f 'Overwrite destination if it exists'
            cand --force 'Overwrite destination if it exists'
            cand --hard-link 'Hard link file'
            cand --symlink 'Symbol link file'
            cand --reflink 'Ref link file Similar to hardlink except modify one doesn''t affect the other'
            cand --verify 'Verify hash of folder / file once copied'
            cand --no-progress 'Disable progress bar'
            cand --no-keep-awake 'Disable keep system awake while copy'
            cand --keep-display-awake 'Keep display awake while copy'
            cand -h 'Print help'
            cand --help 'Print help'
        }
    ]
    $completions[$command]
}
