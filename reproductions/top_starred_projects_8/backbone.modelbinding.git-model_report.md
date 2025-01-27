# Model report for file:///tmp/top-repos-quality-repos-1fxdhitm/backbone.modelbinding.git HEAD ca30d8154bd31c85ccb7134ce9fe55daa5a579ee

### Dump

```json
{'created_at': '2021-08-29 15:59:57',
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
 'size': '17.3 kB',
 'tags': [],
 'uuid': 'd328ea4f-6543-4aa2-9204-fa36a2d78d27',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1fxdhitm/backbone.modelbinding.git ca30d8154bd31c85ccb7134ce9fe55daa5a579ee

# javascript
23 rules, avg.len. 6.6
## train
PPCR: 0.918350
### report
macro
{'f1-score': 0.8172783981883412,
 'precision': 0.827377658586333,
 'recall': 0.8085042905304225,
 'support': 36149}
micro
{'f1-score': 0.9686298376165315,
 'precision': 0.9686298376165315,
 'recall': 0.9686298376165315,
 'support': 36149}
weighted
{'f1-score': 0.966227469092822,
 'precision': 0.9640526517602486,
 'recall': 0.9686298376165315,
 'support': 36149}
### report_full
macro
{'f1-score': 0.7471121411072417,
 'precision': 0.827377658586333,
 'recall': 0.6959024802745066,
 'support': 39363}
micro
{'f1-score': 0.9274022671893208,
 'precision': 0.9686298376165315,
 'recall': 0.8895409394609151,
 'support': 39363}
weighted
{'f1-score': 0.9134876589398264,
 'precision': 0.9450910094358017,
 'recall': 0.8895409394609151,
 'support': 39363}
## test
PPCR: 0.909826
### report
macro
{'f1-score': 0.8250894401658584,
 'precision': 0.8343724585159411,
 'recall': 0.8207473640683536,
 'support': 2250}
micro
{'f1-score': 0.9782222222222222,
 'precision': 0.9782222222222222,
 'recall': 0.9782222222222222,
 'support': 2250}
weighted
{'f1-score': 0.9748007192868996,
 'precision': 0.9725485970797425,
 'recall': 0.9782222222222222,
 'support': 2250}
### report_full
macro
{'f1-score': 0.7786870280245339,
 'precision': 0.8343724585159411,
 'recall': 0.7366695224984884,
 'support': 2473}
micro
{'f1-score': 0.9320347236925683,
 'precision': 0.9782222222222222,
 'recall': 0.8900121310149616,
 'support': 2473}
weighted
{'f1-score': 0.9164271004152589,
 'precision': 0.9519139787687199,
 'recall': 0.8900121310149616,
 'support': 2473}
```

## javascript
### Summary
14 rules, avg.len. 5.8

| | |
|-|-|
|Min support|183|
|Max support|8046|
|Min confidence|0.9252645373344421|
|Max confidence|0.998948872089386|

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
                     'min_samples_split': 195,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 7168.` |
| 2 | `  -1.reserved = (<br>	∧ -4.diff_col ≤ 8<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.981. Support: 243.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1427.` |
| 4 | `  -1.reserved = {<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.982. Support: 908.` |
| 5 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 593.` |
| 6 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 432.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 3882.` |
| 8 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1194.` |
| 9 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.964. Support: 958.` |
| 10 | `  •••start_line ≤ 140<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 183.` |
| 11 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {CALL} and not in {QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 200.` |
| 12 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 756.` |
| 13 | `  -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 321.` |
| 14 | `  -1.reserved not in {if}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 8046.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.785714285714286, "max_conf": 0.998948872089386, "max_support": 8046, "min_conf": 0.9252645373344421, "min_support": 183, "num_rules": 14}}
```
</details>
