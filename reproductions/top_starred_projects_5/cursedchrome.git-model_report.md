# Model report for file:///tmp/top-repos-quality-repos-2rdmnx0m/cursedchrome.git HEAD f18eaee6e3aee866b801235e836d9576aff008de

### Dump

```json
{'created_at': '2021-08-29 22:51:29',
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
 'size': '18.9 kB',
 'tags': [],
 'uuid': 'a753b7eb-3d7f-4cae-be4c-25ea30f887f1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2rdmnx0m/cursedchrome.git f18eaee6e3aee866b801235e836d9576aff008de

# javascript
16 rules, avg.len. 5.8
## train
PPCR: 0.837159
### report
macro
{'f1-score': 0.4476747610030065,
 'precision': 0.45794791986557815,
 'recall': 0.4434043753832312,
 'support': 20898}
micro
{'f1-score': 0.9438702268159632,
 'precision': 0.9438702268159632,
 'recall': 0.9438702268159632,
 'support': 20898}
weighted
{'f1-score': 0.9343956990279197,
 'precision': 0.9269051582674661,
 'recall': 0.9438702268159632,
 'support': 20898}
### report_full
macro
{'f1-score': 0.38790413310970406,
 'precision': 0.45794791986557815,
 'recall': 0.36364918321877776,
 'support': 24963}
micro
{'f1-score': 0.8602080198861778,
 'precision': 0.9438702268159632,
 'recall': 0.790169450787165,
 'support': 24963}
weighted
{'f1-score': 0.8142400405064222,
 'precision': 0.8676003602081476,
 'recall': 0.790169450787165,
 'support': 24963}
## test
PPCR: 0.807387
### report
macro
{'f1-score': 0.39092601470813315,
 'precision': 0.39030446681049485,
 'recall': 0.43770436290856696,
 'support': 2033}
micro
{'f1-score': 0.9006394490900148,
 'precision': 0.9006394490900148,
 'recall': 0.9006394490900148,
 'support': 2033}
weighted
{'f1-score': 0.8871499496028603,
 'precision': 0.8797748114210584,
 'recall': 0.9006394490900148,
 'support': 2033}
### report_full
macro
{'f1-score': 0.3302153460278424,
 'precision': 0.39030446681049485,
 'recall': 0.35157216856552104,
 'support': 2518}
micro
{'f1-score': 0.8046583168534387,
 'precision': 0.9006394490900148,
 'recall': 0.7271644162033359,
 'support': 2518}
weighted
{'f1-score': 0.7553365582771779,
 'precision': 0.821024233332628,
 'recall': 0.7271644162033359,
 'support': 2518}
```

## javascript
### Summary
11 rules, avg.len. 5.4

| | |
|-|-|
|Min support|99|
|Max support|5456|
|Min confidence|0.9242424368858337|
|Max confidence|0.9994292259216309|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 3462.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.958. Support: 318.` |
| 3 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 801.` |
| 4 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 332.` |
| 5 | `  -1.reserved = ,<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 289.` |
| 6 | `  -1.reserved not in {(, ,, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {INITIALIZATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 143.` |
| 7 | `  -1.diff_col ≤ 15<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 99.` |
| 8 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 876.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 712.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 193.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 5456.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.363636363636363, "max_conf": 0.9994292259216309, "max_support": 5456, "min_conf": 0.9242424368858337, "min_support": 99, "num_rules": 11}}
```
</details>
