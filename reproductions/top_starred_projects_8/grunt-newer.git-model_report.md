# Model report for file:///tmp/top-repos-quality-repos-lmoqc1uq/grunt-newer.git HEAD 54484d394f22e2841b94b7fb76eee42a0b4d0f63

### Dump

```json
{'created_at': '2021-08-29 16:20:25',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.6 kB',
 'tags': [],
 'uuid': 'e79a6a12-0f05-4a57-80cc-0d28fc5bebe0',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-lmoqc1uq/grunt-newer.git 54484d394f22e2841b94b7fb76eee42a0b4d0f63

# javascript
57 rules, avg.len. 6.7
## train
PPCR: 0.933193
### report
macro
{'f1-score': 0.6545085561428887,
 'precision': 0.6559097472559731,
 'recall': 0.6567570160674568,
 'support': 7529}
micro
{'f1-score': 0.9148625315446939,
 'precision': 0.9148625315446939,
 'recall': 0.9148625315446939,
 'support': 7529}
weighted
{'f1-score': 0.9007997818196211,
 'precision': 0.8908201744995207,
 'recall': 0.9148625315446939,
 'support': 7529}
### report_full
macro
{'f1-score': 0.6457917291722216,
 'precision': 0.6559097472559731,
 'recall': 0.6394524134738185,
 'support': 8068}
micro
{'f1-score': 0.883246778226582,
 'precision': 0.9148625315446939,
 'recall': 0.8537431829449678,
 'support': 8068}
weighted
{'f1-score': 0.8481357712576396,
 'precision': 0.8472096129316419,
 'recall': 0.8537431829449678,
 'support': 8068}
## test
PPCR: 0.909782
### report
macro
{'f1-score': 0.6517870948268226,
 'precision': 0.6537409956570018,
 'recall': 0.6521922358446659,
 'support': 958}
micro
{'f1-score': 0.8987473903966596,
 'precision': 0.8987473903966597,
 'recall': 0.8987473903966597,
 'support': 958}
weighted
{'f1-score': 0.8792086870198562,
 'precision': 0.8619072809114194,
 'recall': 0.8987473903966597,
 'support': 958}
### report_full
macro
{'f1-score': 0.642822080649266,
 'precision': 0.6537409956570018,
 'recall': 0.634719729600789,
 'support': 1053}
micro
{'f1-score': 0.8562904027846843,
 'precision': 0.8987473903966597,
 'recall': 0.8176638176638177,
 'support': 1053}
weighted
{'f1-score': 0.8106566150488151,
 'precision': 0.8064380470788152,
 'recall': 0.8176638176638177,
 'support': 1053}
```

## javascript
### Summary
26 rules, avg.len. 5.0

| | |
|-|-|
|Min support|143|
|Max support|1712|
|Min confidence|0.9201326370239258|
|Max confidence|0.9988095164299011|

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
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.999. Support: 347.` |
| 2 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 1667.` |
| 3 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.983. Support: 204.` |
| 4 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 1659.` |
| 5 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.997. Support: 143.` |
| 6 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 1712.` |
| 7 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.976. Support: 233.` |
| 8 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.986. Support: 249.` |
| 9 | `  -1.diff_col ≤ 7<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 174.` |
| 10 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 315.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.997. Support: 145.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 1668.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.990. Support: 241.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 147.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 1649.` |
| 16 | `  -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 181.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 366.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 293.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.966. Support: 249.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1671.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.999. Support: 420.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.975. Support: 221.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ␣<br>Confidence: 0.987. Support: 187.` |
| 25 | `  -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 147.` |
| 26 | `  -1.diff_col ≤ 7<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 203.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.0, "max_conf": 0.9988095164299011, "max_support": 1712, "min_conf": 0.9201326370239258, "min_support": 143, "num_rules": 26}}
```
</details>
