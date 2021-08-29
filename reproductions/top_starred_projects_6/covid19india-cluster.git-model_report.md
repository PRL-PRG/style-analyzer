# Model report for file:///tmp/top-repos-quality-repos-j6ce3glj/covid19india-cluster.git HEAD dd60c7ee7054665404ee1a4ff7b7074b4776318e

### Dump

```json
{'created_at': '2021-08-29 21:11:33',
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
 'size': '16.4 kB',
 'tags': [],
 'uuid': '872e94f8-d08f-4bd6-beac-a5b946786817',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-j6ce3glj/covid19india-cluster.git dd60c7ee7054665404ee1a4ff7b7074b4776318e

# javascript
49 rules, avg.len. 6.0
## train
PPCR: 0.847751
### report
macro
{'f1-score': 0.49102492260312375,
 'precision': 0.5019581626824124,
 'recall': 0.4839962434852273,
 'support': 7144}
micro
{'f1-score': 0.8400055991041432,
 'precision': 0.8400055991041433,
 'recall': 0.8400055991041433,
 'support': 7144}
weighted
{'f1-score': 0.8179694713046689,
 'precision': 0.7986492671841029,
 'recall': 0.8400055991041433,
 'support': 7144}
### report_full
macro
{'f1-score': 0.4745286202251614,
 'precision': 0.5019581626824124,
 'recall': 0.4528422774172701,
 'support': 8427}
micro
{'f1-score': 0.7707918566566053,
 'precision': 0.8400055991041433,
 'recall': 0.7121158182033939,
 'support': 8427}
weighted
{'f1-score': 0.7227503277715662,
 'precision': 0.7347521419303419,
 'recall': 0.7121158182033939,
 'support': 8427}
## test
PPCR: 0.845745
### report
macro
{'f1-score': 0.49814464659589985,
 'precision': 0.502103981537769,
 'recall': 0.4967169861036478,
 'support': 1908}
micro
{'f1-score': 0.8312368972746331,
 'precision': 0.8312368972746331,
 'recall': 0.8312368972746331,
 'support': 1908}
weighted
{'f1-score': 0.8102073143265949,
 'precision': 0.7920175858344719,
 'recall': 0.8312368972746331,
 'support': 1908}
### report_full
macro
{'f1-score': 0.48285958348562297,
 'precision': 0.502103981537769,
 'recall': 0.465767763589258,
 'support': 2256}
micro
{'f1-score': 0.7617675312199808,
 'precision': 0.8312368972746331,
 'recall': 0.7030141843971631,
 'support': 2256}
weighted
{'f1-score': 0.7216991645964276,
 'precision': 0.7418841195336976,
 'recall': 0.7030141843971631,
 'support': 2256}
```

## javascript
### Summary
26 rules, avg.len. 5.5

| | |
|-|-|
|Min support|133|
|Max support|888|
|Min confidence|0.9209677577018738|
|Max confidence|0.9979079365730286|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 239.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 436.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 354.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 212.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 156.` |
| 6 | `  -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 330.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 368.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.935. Support: 147.` |
| 9 | `  -1.diff_col ≥ 3<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 310.` |
| 10 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.944. Support: 133.` |
| 11 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 368.` |
| 12 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 271.` |
| 13 | `  -1.label in {<space>}<br>⇒ y = '<br>Confidence: 0.979. Support: 210.` |
| 14 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>⇒ y = '<br>Confidence: 0.997. Support: 197.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = =<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 183.` |
| 16 | `  -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 304.` |
| 17 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 674.` |
| 18 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 192.` |
| 19 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 213.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 888.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 430.` |
| 22 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 373.` |
| 23 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 262.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>⇒ y = '<br>Confidence: 0.969. Support: 208.` |
| 25 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 383.` |
| 26 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 361.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.538461538461538, "max_conf": 0.9979079365730286, "max_support": 888, "min_conf": 0.9209677577018738, "min_support": 133, "num_rules": 26}}
```
</details>
