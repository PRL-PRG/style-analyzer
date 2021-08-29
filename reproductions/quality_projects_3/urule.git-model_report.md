# Model report for file:///tmp/top-repos-quality-repos-5rsjrz5v/urule.git HEAD 1b7edeef8bb6e9b365b249cb9900fe82cf23ad5d

### Dump

```json
{'created_at': '2021-08-29 09:12:28',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '21.3 kB',
 'tags': [],
 'uuid': '67a188d9-baa2-45f8-b1d4-d757afbf6575',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5rsjrz5v/urule.git 1b7edeef8bb6e9b365b249cb9900fe82cf23ad5d

# javascript
48 rules, avg.len. 7.8
## train
PPCR: 0.905242
### report
macro
{'f1-score': 0.741323742912261,
 'precision': 0.8457327428621297,
 'recall': 0.683096837274943,
 'support': 91262}
micro
{'f1-score': 0.9511625868378953,
 'precision': 0.9511625868378953,
 'recall': 0.9511625868378953,
 'support': 91262}
weighted
{'f1-score': 0.9472134644233857,
 'precision': 0.9515965495889829,
 'recall': 0.9511625868378953,
 'support': 91262}
### report_full
macro
{'f1-score': 0.5318588601576257,
 'precision': 0.8457327428621297,
 'recall': 0.4434852607380828,
 'support': 100815}
micro
{'f1-score': 0.9038562659766656,
 'precision': 0.9511625868378953,
 'recall': 0.8610325844368397,
 'support': 100815}
weighted
{'f1-score': 0.8825639688774325,
 'precision': 0.9492539483582111,
 'recall': 0.8610325844368397,
 'support': 100815}
## test
PPCR: 0.915250
### report
macro
{'f1-score': 0.6477390343407252,
 'precision': 0.7304046944674202,
 'recall': 0.6004818513534397,
 'support': 24720}
micro
{'f1-score': 0.9510113268608414,
 'precision': 0.9510113268608414,
 'recall': 0.9510113268608414,
 'support': 24720}
weighted
{'f1-score': 0.9453531390388416,
 'precision': 0.9473558627534581,
 'recall': 0.9510113268608414,
 'support': 24720}
### report_full
macro
{'f1-score': 0.4873848272594023,
 'precision': 0.7304046944674202,
 'recall': 0.41247446573709695,
 'support': 27009}
micro
{'f1-score': 0.9089292273192986,
 'precision': 0.9510113268608414,
 'recall': 0.8704135658484209,
 'support': 27009}
weighted
{'f1-score': 0.8864011364209148,
 'precision': 0.9436195604106883,
 'recall': 0.8704135658484209,
 'support': 27009}
```

## javascript
### Summary
28 rules, avg.len. 7.4

| | |
|-|-|
|Min support|92|
|Max support|45714|
|Min confidence|0.9260563254356384|
|Max confidence|0.9994823932647705|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  •••start_col ≥ 11<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 3076.` |
| 2 | `  -1.reserved not in {;}<br>	∧ -2.label in {<newline>} and not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.951. Support: 175.` |
| 3 | `  •••start_line ≤ 20<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.972. Support: 198.` |
| 4 | `  •••start_col ≤ 12<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.roles in {MAP}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.942. Support: 146.` |
| 5 | `  -1.diff_col ≥ 3<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {BINARY, DECLARATION}<br>⇒ y = "<br>Confidence: 0.996. Support: 120.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -3.roles in {KEY}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {Program}<br>⇒ y = '<br>Confidence: 0.948. Support: 145.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -2.length ≥ 3<br>	∧ -3.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.996. Support: 120.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -2.length ≥ 3<br>	∧ -3.roles not in {KEY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.926. Support: 142.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^2.internal_type = ClassBody<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.996. Support: 129.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^2.internal_type not in {ClassBody}<br>	∧ ^2.roles in {VALUE}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.995. Support: 101.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 966.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 467.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, var, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.992. Support: 310.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 304.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = new<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 203.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = export<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 196.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {LogicalExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, var, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.931. Support: 808.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const, new, return, var, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {LogicalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 3470.` |
| 20 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, var, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 149.` |
| 21 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 143.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = let<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 140.` |
| 23 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 577.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {LogicalExpression}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 1592.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = import<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 94.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 92.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, import, return, var, {}<br>	∧ -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 2490.` |
| 28 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, import, return, var, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {if, {, }}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 45714.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.428571428571429, "max_conf": 0.9994823932647705, "max_support": 45714, "min_conf": 0.9260563254356384, "min_support": 92, "num_rules": 28}}
```
</details>
