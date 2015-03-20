# Replace all `any = [` to `any[] = [`
find *.ts | xargs sed -i 's/any = \[/any\[\] = \[/g'
