# Model report for file:///tmp/top-repos-quality-repos-tet1vdcc/node-rdkafka.git HEAD 3ae6d726a4505ed9ef14c99f67c71a479b5c47d1

### Dump

```json
{'created_at': '2021-08-23 06:18:46',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': '7dd9a9d2-2c52-4724-b89f-c8878bdc02a8',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-tet1vdcc/node-rdkafka.git 3ae6d726a4505ed9ef14c99f67c71a479b5c47d1

# javascript
23 rules, avg.len. 8.0
## train
PPCR: 0.896911
### report
macro
{'f1-score': 0.8102491517920108,
 'precision': 0.8108796359060871,
 'recall': 0.8102194800839034,
 'support': 27006}
micro
{'f1-score': 0.9693401466340813,
 'precision': 0.9693401466340813,
 'recall': 0.9693401466340813,
 'support': 27006}
weighted
{'f1-score': 0.9679984183362423,
 'precision': 0.9669780087309855,
 'recall': 0.9693401466340813,
 'support': 27006}
### report_full
macro
{'f1-score': 0.7353251556432473,
 'precision': 0.8108796359060871,
 'recall': 0.7054730279796919,
 'support': 30110}
micro
{'f1-score': 0.9166608305903775,
 'precision': 0.9693401466340813,
 'recall': 0.8694121554300897,
 'support': 30110}
weighted
{'f1-score': 0.895060253908543,
 'precision': 0.9406440894137199,
 'recall': 0.8694121554300897,
 'support': 30110}
## test
PPCR: 0.882310
### report
macro
{'f1-score': 0.7976720160277722,
 'precision': 0.7833014931148082,
 'recall': 0.8131057476842413,
 'support': 6110}
micro
{'f1-score': 0.9594108019639934,
 'precision': 0.9594108019639934,
 'recall': 0.9594108019639934,
 'support': 6110}
weighted
{'f1-score': 0.9578372444462186,
 'precision': 0.9565827041681266,
 'recall': 0.9594108019639934,
 'support': 6110}
### report_full
macro
{'f1-score': 0.7175275351247076,
 'precision': 0.7833014931148082,
 'recall': 0.7030405115783271,
 'support': 6925}
micro
{'f1-score': 0.8994246260069044,
 'precision': 0.9594108019639934,
 'recall': 0.8464981949458483,
 'support': 6925}
weighted
{'f1-score': 0.8751881289974024,
 'precision': 0.9225278468398893,
 'recall': 0.8464981949458483,
 'support': 6925}
```

## javascript
### Summary
18 rules, avg.len. 7.9

| | |
|-|-|
|Min support|126|
|Max support|8522|
|Min confidence|0.9280973672866821|
|Max confidence|0.999493420124054|

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
                     'min_samples_leaf': 120,
                     'min_samples_split': 234,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 987.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.994. Support: 595.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved not in {:}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 180.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.996. Support: 126.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.954. Support: 830.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {FILE, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 862.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.973. Support: 823.` |
| 8 | `  •••start_col ≥ 36<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 133.` |
| 9 | `  •••start_col ≤ 35<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 879.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 770.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 409.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 389.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 155.` |
| 14 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, var, {}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 146.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, var, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 452.` |
| 16 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, var, {}<br>	∧ -1.length ≥ 3<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 203.` |
| 17 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, var, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 3243.` |
| 18 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, var, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 8522.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.888888888888889, "max_conf": 0.999493420124054, "max_support": 8522, "min_conf": 0.9280973672866821, "min_support": 126, "num_rules": 18}}
```
</details>
