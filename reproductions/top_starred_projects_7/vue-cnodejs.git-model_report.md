# Model report for file:///tmp/top-repos-quality-repos-d8svls6c/vue-cnodejs.git HEAD b23c152b13839da2bafa7388780f17e433d4a4ce

### Dump

```json
{'created_at': '2021-08-31 14:06:35',
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
 'size': '15.3 kB',
 'tags': [],
 'uuid': '28096423-8a6e-46ea-a3dc-ed9d9a30150e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-d8svls6c/vue-cnodejs.git b23c152b13839da2bafa7388780f17e433d4a4ce

# javascript
14 rules, avg.len. 5.4
## train
PPCR: 0.816081
### report
macro
{'f1-score': 0.9035774334575685,
 'precision': 0.9415741135828283,
 'recall': 0.88187728749583,
 'support': 3390}
micro
{'f1-score': 0.9392330383480826,
 'precision': 0.9392330383480826,
 'recall': 0.9392330383480826,
 'support': 3390}
weighted
{'f1-score': 0.9373468204946064,
 'precision': 0.9417647307781158,
 'recall': 0.9392330383480826,
 'support': 3390}
### report_full
macro
{'f1-score': 0.8061309345348588,
 'precision': 0.9415741135828283,
 'recall': 0.7324111738029498,
 'support': 4154}
micro
{'f1-score': 0.8441145281018028,
 'precision': 0.9392330383480826,
 'recall': 0.7664901299951854,
 'support': 4154}
weighted
{'f1-score': 0.835200845451498,
 'precision': 0.9398467109627057,
 'recall': 0.7664901299951854,
 'support': 4154}
## test
PPCR: 0.840149
### report
macro
{'f1-score': 0.8836782019193662,
 'precision': 0.9150686631204396,
 'recall': 0.8653904727095516,
 'support': 226}
micro
{'f1-score': 0.9336283185840708,
 'precision': 0.9336283185840708,
 'recall': 0.9336283185840708,
 'support': 226}
weighted
{'f1-score': 0.9343609704281959,
 'precision': 0.9383138628757974,
 'recall': 0.9336283185840708,
 'support': 226}
### report_full
macro
{'f1-score': 0.7725516638833239,
 'precision': 0.9150686631204396,
 'recall': 0.6850769189283933,
 'support': 269}
micro
{'f1-score': 0.8525252525252525,
 'precision': 0.9336283185840708,
 'recall': 0.7843866171003717,
 'support': 269}
weighted
{'f1-score': 0.8392884163530149,
 'precision': 0.9243256661093947,
 'recall': 0.7843866171003717,
 'support': 269}
```

## javascript
### Summary
10 rules, avg.len. 4.8

| | |
|-|-|
|Min support|90|
|Max support|547|
|Min confidence|0.9301075339317322|
|Max confidence|0.9977578520774841|

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
                     'min_samples_split': 183,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 223.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.996. Support: 117.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 420.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 133.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.934. Support: 114.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 93.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.942. Support: 112.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 146.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 547.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 90.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.8, "max_conf": 0.9977578520774841, "max_support": 547, "min_conf": 0.9301075339317322, "min_support": 90, "num_rules": 10}}
```
</details>
