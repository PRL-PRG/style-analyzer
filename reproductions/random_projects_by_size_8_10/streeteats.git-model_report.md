# Model report for file:///tmp/top-repos-quality-repos-_zxoy3vj/streeteats.git HEAD c532ee6e58af60f3453757bf1867a52f70438b6c

### Dump

```json
{'created_at': '2021-08-21 00:02:45',
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
 'size': '16.6 kB',
 'tags': [],
 'uuid': 'ce719a03-ce46-4716-8576-47ca859671bb',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_zxoy3vj/streeteats.git c532ee6e58af60f3453757bf1867a52f70438b6c

# javascript
33 rules, avg.len. 4.8
## train
PPCR: 0.984809
### report
macro
{'f1-score': 0.7497032810029076,
 'precision': 0.785563399472737,
 'recall': 0.7361434413326366,
 'support': 6483}
micro
{'f1-score': 0.8587073885546814,
 'precision': 0.8587073885546814,
 'recall': 0.8587073885546814,
 'support': 6483}
weighted
{'f1-score': 0.8341923658039396,
 'precision': 0.8319482041935873,
 'recall': 0.8587073885546814,
 'support': 6483}
### report_full
macro
{'f1-score': 0.7468022495831192,
 'precision': 0.785563399472737,
 'recall': 0.7302391116343302,
 'support': 6583}
micro
{'f1-score': 0.8521353130261748,
 'precision': 0.8587073885546814,
 'recall': 0.8456630715479265,
 'support': 6583}
weighted
{'f1-score': 0.8254775083815385,
 'precision': 0.8262275655565811,
 'recall': 0.8456630715479265,
 'support': 6583}
## test
PPCR: 0.994660
### report
macro
{'f1-score': 0.7365105943492825,
 'precision': 0.8012042029746035,
 'recall': 0.721372503186073,
 'support': 10244}
micro
{'f1-score': 0.9066770792659118,
 'precision': 0.9066770792659118,
 'recall': 0.9066770792659118,
 'support': 10244}
weighted
{'f1-score': 0.892778814005327,
 'precision': 0.9031046919076167,
 'recall': 0.9066770792659118,
 'support': 10244}
### report_full
macro
{'f1-score': 0.7358852071884404,
 'precision': 0.8012042029746035,
 'recall': 0.7199573991756365,
 'support': 10299}
micro
{'f1-score': 0.9042496227425401,
 'precision': 0.9066770792659118,
 'recall': 0.9018351296242354,
 'support': 10299}
weighted
{'f1-score': 0.8896885200031485,
 'precision': 0.901261235993955,
 'recall': 0.9018351296242354,
 'support': 10299}
```

## javascript
### Summary
13 rules, avg.len. 4.2

| | |
|-|-|
|Min support|152|
|Max support|743|
|Min confidence|0.9557692408561707|
|Max confidence|0.9992647171020508|

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
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 202,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.999. Support: 680.` |
| 2 | `  -1.reserved = ,<br>	∧ +1.length ≥ 4<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 476.` |
| 3 | `  -1.reserved not in {,, :}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.979. Support: 268.` |
| 4 | `  -1.reserved = ,<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 441.` |
| 5 | `  -1.reserved not in {,, :}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.985. Support: 229.` |
| 6 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.970. Support: 152.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.974. Support: 250.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.997. Support: 159.` |
| 9 | `  -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 743.` |
| 10 | `  -1.reserved = ,<br>	∧ +1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 433.` |
| 11 | `  -1.reserved = ,<br>	∧ +1.roles in {KEY}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 449.` |
| 12 | `  -1.reserved not in {,, :}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 728.` |
| 13 | `  -1.reserved not in {,, :}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {CALL} and not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 260.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.153846153846154, "max_conf": 0.9992647171020508, "max_support": 743, "min_conf": 0.9557692408561707, "min_support": 152, "num_rules": 13}}
```
</details>
