# Model report for file:///tmp/top-repos-quality-repos-_b5lskm2/perspective.git HEAD ef1690e7a9474ccd93243fd80ca48010478166fb

### Dump

```json
{'created_at': '2021-08-23 05:31:32',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '24.3 kB',
 'tags': [],
 'uuid': 'ea528e46-16b9-4aa6-abca-9506fbe281af',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_b5lskm2/perspective.git ef1690e7a9474ccd93243fd80ca48010478166fb

# javascript
316 rules, avg.len. 10.9
## train
PPCR: 0.987546
### report
macro
{'f1-score': 0.6997970868979464,
 'precision': 0.7181269449850584,
 'recall': 0.6871582193336133,
 'support': 225666}
micro
{'f1-score': 0.9605345953754664,
 'precision': 0.9605345953754664,
 'recall': 0.9605345953754664,
 'support': 225666}
weighted
{'f1-score': 0.9587171902203223,
 'precision': 0.9576789492532382,
 'recall': 0.9605345953754664,
 'support': 225666}
### report_full
macro
{'f1-score': 0.6813510638519752,
 'precision': 0.7181269449850584,
 'recall': 0.6586168770741804,
 'support': 228512}
micro
{'f1-score': 0.9545156304356441,
 'precision': 0.9605345953754664,
 'recall': 0.9485716286234421,
 'support': 228512}
weighted
{'f1-score': 0.9515362733563483,
 'precision': 0.9568714605600699,
 'recall': 0.9485716286234421,
 'support': 228512}
## test
PPCR: 0.986211
### report
macro
{'f1-score': 0.6798034969117821,
 'precision': 0.691545368920595,
 'recall': 0.6743874395593722,
 'support': 60723}
micro
{'f1-score': 0.9566885694053324,
 'precision': 0.9566885694053324,
 'recall': 0.9566885694053324,
 'support': 60723}
weighted
{'f1-score': 0.9554205160420939,
 'precision': 0.9551407323694964,
 'recall': 0.9566885694053324,
 'support': 60723}
### report_full
macro
{'f1-score': 0.6611610993017335,
 'precision': 0.691545368920595,
 'recall': 0.6411771707688673,
 'support': 61572}
micro
{'f1-score': 0.9500470174577865,
 'precision': 0.9566885694053324,
 'recall': 0.9434970441109596,
 'support': 61572}
weighted
{'f1-score': 0.9476345291971109,
 'precision': 0.9533834573940352,
 'recall': 0.9434970441109596,
 'support': 61572}
```

## javascript
### Summary
204 rules, avg.len. 11.2

| | |
|-|-|
|Min support|142|
|Max support|64691|
|Min confidence|0.9202573895454407|
|Max confidence|0.9998274445533752|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.979. Support: 8656.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1887.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 221.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {MAP}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 214.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 1632.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.950. Support: 803.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, [}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.982. Support: 751.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.965. Support: 7710.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 1.000. Support: 2508.` |
| 10 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.length ≥ 7<br>⇒ y = ⏎⏎<br>Confidence: 0.923. Support: 733.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.978. Support: 8179.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^2.roles in {LIST, MAP}<br>⇒ y = ⏎<br>Confidence: 0.923. Support: 462.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 6395.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 221.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 165.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {]}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 5780.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 4079.` |
| 18 | `  •••start_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.977. Support: 3319.` |
| 19 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 288.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2898.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 822.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 3346.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.952. Support: 1644.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.length ≤ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 1144.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.920. Support: 1787.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -1.length ≥ 3<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 3<br>⇒ y = ␣<br>Confidence: 0.992. Support: 2849.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.length ≤ 2<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type = IfStatement<br>⇒ y = ␣<br>Confidence: 0.998. Support: 260.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1238.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 2<br>⇒ y = ␣<br>Confidence: 0.999. Support: 773.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.998. Support: 213.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 8612.` |
| 32 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≥ 4<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {], }}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.965. Support: 327.` |
| 33 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 3042.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.reserved = =<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 715.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = async<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 595.` |
| 36 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 476.` |
| 37 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>⇒ y = ␣<br>Confidence: 0.991. Support: 395.` |
| 38 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = new<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 384.` |
| 39 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = from<br>⇒ y = ␣<br>Confidence: 0.987. Support: 339.` |
| 40 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {ITERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 244.` |
| 41 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {ITERATOR} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 178.` |
| 42 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.reserved not in {"}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +3.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1501.` |
| 43 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const, {, }}<br>	∧ -2.reserved not in {", =}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +3.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 5603.` |
| 44 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {MODULE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 971.` |
| 45 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.length ≤ 1<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 1404.` |
| 46 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved = =<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {IfStatement, VariableDeclarator}<br>	∧ ^1.roles in {EXPRESSION} and not in {MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1713.` |
| 47 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {IfStatement, VariableDeclarator}<br>	∧ ^1.roles not in {MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 64691.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 8<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 165.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1563.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = from<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.998. Support: 283.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.994. Support: 262.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, from}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 261.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.947. Support: 8507.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {SCOPE}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 158.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.973. Support: 8458.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +5.reserved = ,<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 355.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 6348.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 235.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 175.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {]}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 5440.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 4053.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.957. Support: 197.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.966. Support: 3671.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 904.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1535.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.length ≤ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 1107.` |
| 67 | `  •••start_col ≤ 68<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.974. Support: 1720.` |
| 68 | `  -1.diff_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.length ≥ 85<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 340.` |
| 69 | `  -1.diff_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.953. Support: 4697.` |
| 70 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 203.` |
| 71 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles in {STATEMENT} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 530.` |
| 72 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {, }}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.932. Support: 406.` |
| 73 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {, }}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.997. Support: 157.` |
| 74 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {=}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {ITERATOR, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 1625.` |
| 75 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {ITERATOR, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 19664.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.999. Support: 849.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>⇒ y = ␣<br>Confidence: 0.991. Support: 372.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.991. Support: 290.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 1697.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 42099.` |
| 81 | `  -1.diff_col ≥ 3<br>	∧ -1.diff_offset ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 687.` |
| 82 | `  -1.diff_col ≥ 3<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {], }}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.941. Support: 635.` |
| 83 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 23200.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1551.` |
| 85 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 174.` |
| 86 | `  •••start_col ≤ 66<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {NUMBER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ +3.roles not in {LITERAL, NUMBER}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ⏎<br>Confidence: 0.957. Support: 1062.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +5.reserved = ,<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 302.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {=}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {]}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 4786.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≥ 7<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 622.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_line = 0<br>	∧ -5.diff_offset ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles in {MAP}<br>	∧ +5.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 197.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.962. Support: 3667.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +2.reserved = ><br>⇒ y = ␣<br>Confidence: 1.000. Support: 1475.` |
| 93 | `  -1.diff_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.length ≥ 88<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 364.` |
| 94 | `  -1.diff_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.length ≤ 87<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {STRING} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 164.` |
| 95 | `  -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.949. Support: 4658.` |
| 96 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = TemplateLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 238.` |
| 97 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 563.` |
| 98 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.diff_col ≤ 2<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {=}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 1627.` |
| 99 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 20027.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -2.length ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.945. Support: 266.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -2.length ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 1800.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 41985.` |
| 103 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 678.` |
| 104 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles not in {CALL}<br>	∧ -4.roles in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +1.length ≤ 1<br>	∧ +4.roles in {IDENTIFIER} and not in {STRING}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 157.` |
| 105 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +1.length ≤ 1<br>	∧ +4.roles not in {IDENTIFIER, STRING}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 745.` |
| 106 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 23596.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 8<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 142.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1566.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {ObjectExpression, VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 6287.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {]}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 5509.` |
| 111 | `  •••start_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.928. Support: 244.` |
| 112 | `  •••start_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.959. Support: 3672.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.length ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 1002.` |
| 114 | `  •••start_col ≤ 51<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.964. Support: 1533.` |
| 115 | `  -1.diff_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.length ≥ 88<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 396.` |
| 116 | `  -1.diff_col ≥ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.length ≤ 87<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {LITERAL} and not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 148.` |
| 117 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 206.` |
| 118 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -1.length ≤ 1<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {=}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {ITERATOR, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 1670.` |
| 119 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {ITERATOR, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 20078.` |
| 120 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -2.length ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1852.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, [, from}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.934. Support: 388.` |
| 122 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 153.` |
| 123 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 1280.` |
| 124 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 6384.` |
| 125 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 5763.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_line = 0<br>	∧ -5.diff_offset ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles in {MAP}<br>	∧ +5.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 193.` |
| 127 | `  •••start_col ≤ 72<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.971. Support: 1672.` |
| 128 | `  -1.diff_col ≥ 10<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 505.` |
| 129 | `  -1.diff_col ≥ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 171.` |
| 130 | `  -1.diff_col ≥ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 603.` |
| 131 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 209.` |
| 132 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 547.` |
| 133 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 21717.` |
| 134 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -2.length ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.929. Support: 289.` |
| 135 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -2.length ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 1864.` |
| 136 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 603.` |
| 137 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {MAP}<br>	∧ ^2.roles not in {LIST}<br>⇒ y = ⏎<br>Confidence: 0.957. Support: 198.` |
| 138 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {RIGHT}<br>⇒ y = "<br>Confidence: 0.992. Support: 301.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, from}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {RIGHT}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 273.` |
| 140 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.951. Support: 8506.` |
| 141 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.928. Support: 1872.` |
| 142 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1357.` |
| 143 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.998. Support: 213.` |
| 144 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.length ≥ 7<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 197.` |
| 145 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 8517.` |
| 146 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {], }}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 267.` |
| 147 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 2916.` |
| 148 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 470.` |
| 149 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {ITERATOR} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 170.` |
| 150 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], }}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 1195.` |
| 151 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -3.diff_line = 0<br>	∧ -3.roles not in {STRING}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +3.reserved = (<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1496.` |
| 152 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -3.roles not in {STRING}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +3.reserved = (<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 5515.` |
| 153 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles in {MODULE} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 985.` |
| 154 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.length ≤ 1<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {MODULE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 1440.` |
| 155 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved = =<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles in {EXPRESSION} and not in {MODULE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1603.` |
| 156 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {MODULE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 64521.` |
| 157 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.991. Support: 158.` |
| 158 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {BLOCK}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 204.` |
| 159 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.roles in {CALL}<br>⇒ y = ⏎⏎<br>Confidence: 0.974. Support: 518.` |
| 160 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.roles not in {CALL}<br>	∧ ^1.roles in {MODULE}<br>⇒ y = ⏎⏎<br>Confidence: 0.949. Support: 166.` |
| 161 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +5.reserved = ,<br>	∧ ^1.internal_type not in {ObjectExpression, VariableDeclarator}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 362.` |
| 162 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {]}<br>	∧ +1.roles in {RIGHT}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 305.` |
| 163 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_line = 0<br>	∧ -5.diff_offset ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles in {MAP}<br>	∧ +5.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 185.` |
| 164 | `  -1.diff_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.length ≥ 77<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 338.` |
| 165 | `  -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 152.` |
| 166 | `  -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.969. Support: 4434.` |
| 167 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>⇒ y = ␣<br>Confidence: 0.998. Support: 208.` |
| 168 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.921. Support: 385.` |
| 169 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IMPORT}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 321.` |
| 170 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.label in {<space>} and not in {<-space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.998. Support: 251.` |
| 171 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -2.label in {<space>} and not in {<-space>}<br>	∧ -2.reserved not in {=}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IMPORT, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 1736.` |
| 172 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IMPORT, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 20276.` |
| 173 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1693.` |
| 174 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.roles in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +1.length ≤ 1<br>	∧ +4.internal_type not in {StringLiteral}<br>	∧ +4.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 160.` |
| 175 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +1.length ≤ 1<br>	∧ +4.internal_type not in {StringLiteral}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 810.` |
| 176 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, from}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {BINARY}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 269.` |
| 177 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {BINARY}<br>⇒ y = "<br>Confidence: 0.986. Support: 315.` |
| 178 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {BLOCK}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.965. Support: 156.` |
| 179 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {]}<br>	∧ +1.roles in {BINARY}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 348.` |
| 180 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {]}<br>	∧ +1.roles not in {BINARY}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 149.` |
| 181 | `  -1.diff_col ≤ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -1.length ≤ 2<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1291.` |
| 182 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {], }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 271.` |
| 183 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +3.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1582.` |
| 184 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +3.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 5650.` |
| 185 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +5.reserved = ,<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 368.` |
| 186 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≥ 7<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 629.` |
| 187 | `  -1.diff_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.length ≥ 87<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 407.` |
| 188 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STRING} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 225.` |
| 189 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 519.` |
| 190 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -2.length ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1807.` |
| 191 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.935. Support: 238.` |
| 192 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 3<br>⇒ y = ␣<br>Confidence: 0.993. Support: 2978.` |
| 193 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type = IfStatement<br>⇒ y = ␣<br>Confidence: 0.998. Support: 242.` |
| 194 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1253.` |
| 195 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 1559.` |
| 196 | `  -1.diff_offset ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 187.` |
| 197 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 853.` |
| 198 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 8821.` |
| 199 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +3.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1562.` |
| 200 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +3.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 5616.` |
| 201 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {MODULE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 956.` |
| 202 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.length ≤ 1<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 1402.` |
| 203 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved = =<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {IfStatement, VariableDeclarator}<br>	∧ ^1.roles in {EXPRESSION} and not in {MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1716.` |
| 204 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {, }}<br>	∧ -2.reserved not in {=}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {IfStatement, VariableDeclarator}<br>	∧ ^1.roles not in {MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 64466.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 11.230392156862745, "max_conf": 0.9998274445533752, "max_support": 64691, "min_conf": 0.9202573895454407, "min_support": 142, "num_rules": 204}}
```
</details>
